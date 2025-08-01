// src/main.rs
use image::{GenericImageView, ImageReader};
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;

struct AsciiConverter {
    width: u32,
    height: u32,
    chars: Vec<char>,
}

impl AsciiConverter {
    fn new(width: u32, height: u32) -> Self {
        // ASCII characters from dark to light - using only Kindle-confirmed characters
        // Removed "#" and other unavailable chars, using only tested ones
    let chars = vec![
        '.', ',', '-', '_', ':', '=', '+', 'i', 'l', '1', 
        't', 'f', 'j', 'r', 'c', '*', '?', 'v', 'n', 'u', 
        'e', 'o', 'a', 'y', 'x', 'z', 's', '7', '3', '2', 
        '5', 'L', 'T', 'J', 'F', 'C', 'I', 'Y', 'Z', 'E', 
        'S', 'P', '6', '9', '4', 'h', 'k', 'd', 'q', 'w', 
        '[', ']', 'V', 'p', 'G', 'b', 'A', 'K', 'X', 'H', 
        'U', '8', '0', 'O', 'R', 'D', 'B', 'g', 'm', 'M', 
        'N', 'W', 'Q', '@'
    ];
        
        AsciiConverter {
            width,
            height, 
            chars,
        }
    }
   
    fn enhance_contrast(gray: u8, contrast: f32) -> u8 {
        let normalized = gray as f32 / 255.0;
        let enhanced = ((normalized - 0.5) * contrast + 0.5).clamp(0.0, 1.0);
        (enhanced * 255.0) as u8
    }



    fn image_to_ascii(&self, image_path: &str) -> io::Result<String> {
        // Load and process image
        let img = match ImageReader::open(image_path) {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Error decoding {}: {}", image_path, e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, e));
                }
            },
            Err(e) => {
                eprintln!("Error opening {}: {}", image_path, e);
                return Err(io::Error::new(io::ErrorKind::NotFound, e));
            }
        };
        
        // Resize image to target dimensions
        let resized = img.resize_exact(self.width, self.height, image::imageops::FilterType::Lanczos3);
        
        // Convert to grayscale and ASCII
        let mut ascii_output = String::new();
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = resized.get_pixel(x, y);
                
             // Better grayscale conversion using luminance
                let gray = (0.299 * pixel[0] as f32 + 
                       0.587 * pixel[1] as f32 + 
                       0.114 * pixel[2] as f32) as u8;   

                // Apply contrast enhancement for better ASCII mapping
                let enhanced_gray = Self::enhance_contrast(gray, 1.2); // Adjust contrast factor as needed 

            // Map grayscale to ASCII character
                let char_index = (enhanced_gray as usize * (self.chars.len() - 1)) / 255;
                let char_index = char_index.min(self.chars.len() - 1);
                
                ascii_output.push(self.chars[char_index]);
            }
            ascii_output.push('\n');
        }
        
        Ok(ascii_output)
    }
}

struct KindlePlayer {
    converter: AsciiConverter,
}

impl KindlePlayer {
    fn new() -> Self {
        // Kindle-optimized dimensions: 60 wide x 39 high (using full screen height)
        let converter = AsciiConverter::new(50, 40);
        
        KindlePlayer { converter }
    }
    
    fn clear_screen(&self) -> io::Result<()> {
        Command::new("eips")
            .arg("-c")
            .status()?;
        Ok(())
    }
   
    fn display_ascii(&self, ascii_content: &str, previous_frame: Option<&str>) -> io::Result<()> {
        let lines: Vec<&str> = ascii_content.lines().collect();
        let prev_lines: Vec<&str> = previous_frame
            .map(|p| p.lines().collect())
            .unwrap_or_else(Vec::new);
        
        for (line_num, line) in lines.iter().enumerate() {
            if line_num >= 40 {
                break;
            }
            
            // Skip if line is identical to previous frame
            if line_num < prev_lines.len() && prev_lines[line_num] == *line {
                continue;
            }
            
            let display_line = if line.len() > 60 {
                &line[..60]
            } else {
                line
            };
            
            if !display_line.trim().is_empty() {
                Command::new("eips")
                    .arg("0")
                    .arg(line_num.to_string())
                    .arg(display_line)
                    .status()?;
            }
        }
        
        Ok(())
    }

    fn find_frame_files(&self, directory: &str) -> io::Result<Vec<String>> {
        let mut frame_files = Vec::new();
        
        let entries = fs::read_dir(directory)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name() {
                if let Some(name_str) = filename.to_str() {
                    if name_str.starts_with("test_frame_") && 
                       (name_str.ends_with(".png") || name_str.ends_with(".jpg")) {
                        frame_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        // Sort frames in numerical order
        frame_files.sort();
        
        println!("Found {} image frames", frame_files.len());
        
        Ok(frame_files)
    }
    
    fn convert_and_play(&self, directory: &str, loops: u32, save_ascii: bool) -> io::Result<()> {
        let frame_files = self.find_frame_files(directory)?;
        
        if frame_files.is_empty() {
            println!("No frame files found in {}", directory);
            return Ok(());
        }
        
        println!("🎬 Converting {} frames to ASCII and playing...", frame_files.len());
        
        // Convert all frames to ASCII first (if saving) or convert on-the-fly
        let mut ascii_frames = Vec::new();
        
        if save_ascii {
            println!("📝 Converting all frames to ASCII...");
            for (i, frame_file) in frame_files.iter().enumerate() {
                print!("\rConverting frame {} of {}...", i + 1, frame_files.len());
                
                match self.converter.image_to_ascii(frame_file) {
                    Ok(ascii) => ascii_frames.push(ascii),
                    Err(e) => {
                        eprintln!("\nError converting {}: {}", frame_file, e);
                        continue;
                    }
                }
            }
            println!("\n✅ ASCII conversion complete!");
        }
        
        // Clear screen once at the very start, then let frames overlay
        self.clear_screen()?;
        
        // Play animation with ghosting effect
        for loop_num in 0..loops {
            if loops > 1 {
                println!("🔄 Loop {} of {}", loop_num + 1, loops);
            }

            let mut previous_frame: Option<String> = None;
            
            for (i, frame_file) in frame_files.iter().enumerate() {
                print!("\rPlaying frame {} of {}...", i + 1, frame_files.len());
                
                // Get ASCII content
                let ascii_content = if save_ascii {
                    &ascii_frames[i]
                } else {
                    // Convert on-the-fly to save memory
                    match self.converter.image_to_ascii(frame_file) {
                        Ok(ascii) => {
                            // We need to store this temporarily
                            ascii_frames.push(ascii);
                            &ascii_frames[ascii_frames.len() - 1]
                        },
                        Err(e) => {
                            eprintln!("\nError converting {}: {}", frame_file, e);
                            continue;
                        }
                    }
                };
                
                // Display frame directly - no clearing for ghosting effect
                self.display_ascii(ascii_content, previous_frame.as_deref())?;
                

                // Store current frame as previous for next iteration
                previous_frame = Some(ascii_content.to_string());
            }
        }
        
        println!("\n✨ ASCII video playback complete!");
        Ok(())
    }
    
    fn convert_frame_by_frame(&self, directory: &str) -> io::Result<()> {
        let frame_files = self.find_frame_files(directory)?;
        
        if frame_files.is_empty() {
            println!("No frame files found in {}", directory);
            return Ok(());
        }
        
        println!("🎬 Frame-by-frame mode (press Enter for next frame, 'q' to quit)");
        
        // Clear once at start
        self.clear_screen()?;
        
        for (i, frame_file) in frame_files.iter().enumerate() {
            println!("Frame {} of {}: {}", i + 1, frame_files.len(), frame_file);
            
            match self.converter.image_to_ascii(frame_file) {
                Ok(ascii_content) => {
                    // No clearing between frames - overlay for ghosting effect
                    self.display_ascii(&ascii_content, None)?;
                },
                Err(e) => {
                    eprintln!("Error converting {}: {}", frame_file, e);
                    continue;
                }
            }
            
            // Wait for user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if input.trim() == "q" {
                break;
            }
        }
        
        Ok(())
    }
}


fn main() -> io::Result<()> {
    println!("🎭 Kindle ASCII Video Player");
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  {} <image_directory> [loops]        - Convert and play video", args[0]);
        println!("  {} <image_directory> step           - Frame-by-frame mode", args[0]);
        println!("  {} <image_directory> save [loops]   - Pre-convert all frames", args[0]);
        return Ok(());
    }
    
    let image_dir = &args[1];
    
    if !Path::new(image_dir).exists() {
        println!("Directory {} does not exist!", image_dir);
        return Ok(());
    }
    
    let player = KindlePlayer::new();
    
    if args.len() >= 3 {
        match args[2].as_str() {
            "step" => {
                player.convert_frame_by_frame(image_dir)?;
            },
            "save" => {
                let loops = if args.len() >= 4 {
                    args[3].parse().unwrap_or(1)
                } else {
                    1
                };
                player.convert_and_play(image_dir, loops, true)?; // save=true
            },
            _ => {
                let loops = args[2].parse().unwrap_or(1);
                player.convert_and_play(image_dir, loops, false)?; // save=false
            }
        }
    } else {
        player.convert_and_play(image_dir, 1, false)?; // save=false
    }
    
    Ok(())
}
