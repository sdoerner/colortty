#[cfg(test)]
mod color_tests {
    mod color {
        use colortty::Color;

        #[test]
        fn from_mintty_color_works() {
            assert_eq!(
                Color::from_mintty_color("12,3,255").unwrap(),
                Color {
                    red: 12,
                    green: 3,
                    blue: 255
                }
            );
        }

        #[test]
        fn from_mintty_color_invalid_format() {
            assert!(Color::from_mintty_color("123").is_err());
        }

        #[test]
        fn from_mintty_color_parse_int_error() {
            assert!(Color::from_mintty_color("abc,3,fo").is_err());
        }

        #[test]
        fn to_hex() {
            assert_eq!(
                Color {
                    red: 123,
                    green: 4,
                    blue: 255
                }
                .to_hex(),
                "0x7b04ff"
            );
        }
    }

    mod color_scheme {
        use colortty::ColorScheme;
        use std::fs::File;
        use std::io::Read;

        fn read_fixture(filename: &str) -> String {
            let mut fixture = String::new();
            File::open(filename)
                .unwrap()
                .read_to_string(&mut fixture)
                .unwrap();
            return fixture;
        }

        #[test]
        fn convert_minttyrc() {
            let dracula_minttyrc = read_fixture("tests/fixtures/Dracula.minttyrc");
            let dracula_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0x282a36'
    foreground: '0xf8f8f2'

  # Normal colors
  normal:
    black:   '0x000000'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xcaa9fa'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xbfbfbf'

  # Bright colors
  bright:
    black:   '0x282a35'
    red:     '0xff6e67'
    green:   '0x5af78e'
    yellow:  '0xf4f99d'
    blue:    '0xcaa9fa'
    magenta: '0xff92d0'
    cyan:    '0x9aedfe'
    white:   '0xe6e6e6'
"
            .to_string();
            let scheme = ColorScheme::from_minttyrc(&dracula_minttyrc).unwrap();
            assert_eq!(scheme.to_yaml(), dracula_alacritty);
        }

        #[test]
        fn convert_iterm() {
            let dracula_iterm = read_fixture("tests/fixtures/Dracula.itermcolors");
            let dracula_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0x1e1f28'
    foreground: '0xf8f8f2'

  # Cursor colors
  cursor:
    text:   '0xffffff'
    cursor: '0xbbbbbb'

  # Normal colors
  normal:
    black:   '0x000000'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xbd93f9'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xbbbbbb'

  # Bright colors
  bright:
    black:   '0x555555'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xf1fa8c'
    blue:    '0xbd93f9'
    magenta: '0xff79c6'
    cyan:    '0x8be9fd'
    white:   '0xffffff'
"
            .to_string();
            let scheme = ColorScheme::from_iterm(&dracula_iterm).unwrap();
            assert_eq!(scheme.to_yaml(), dracula_alacritty);
        }

        #[test]
        fn convert_iterm_complicated() {
            let firewatch_iterm = read_fixture("tests/fixtures/two-firewatch-light.itermcolors");
            let scheme = ColorScheme::from_iterm(&firewatch_iterm).unwrap();
            let firewatch_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0xf8f6f2'
    foreground: '0x75541b'

  # Cursor colors
  cursor:
    text:   '0xd5deff'
    cursor: '0xda4181'

  # Normal colors
  normal:
    black:   '0x383a42'
    red:     '0xe45649'
    green:   '0x50a14f'
    yellow:  '0xc18401'
    blue:    '0x0184bc'
    magenta: '0xa626a4'
    cyan:    '0x0997b3'
    white:   '0xfafafa'

  # Bright colors
  bright:
    black:   '0x4f525e'
    red:     '0xe06c75'
    green:   '0x98c379'
    yellow:  '0xe5c07b'
    blue:    '0x61afef'
    magenta: '0xc678dd'
    cyan:    '0x56b6c2'
    white:   '0xffffff'
"
            .to_string();
            assert_eq!(scheme.to_yaml(), firewatch_alacritty);
        }

        #[test]
        fn convert_gogh() {
            let dracula_gogh = read_fixture("tests/fixtures/dracula.sh");
            let dracula_alacritty: String = "colors:
  # Default colors
  primary:
    background: '0x282a36'
    foreground: '0x94a3a5'

  # Normal colors
  normal:
    black:   '0x44475a'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xffb86c'
    blue:    '0x8be9fd'
    magenta: '0xbd93f9'
    cyan:    '0xff79c6'
    white:   '0x94a3a5'

  # Bright colors
  bright:
    black:   '0x000000'
    red:     '0xff5555'
    green:   '0x50fa7b'
    yellow:  '0xffb86c'
    blue:    '0x8be9fd'
    magenta: '0xbd93f9'
    cyan:    '0xff79c6'
    white:   '0xffffff'
"
            .to_string();
            let scheme = ColorScheme::from_gogh(&dracula_gogh).unwrap();
            assert_eq!(scheme.to_yaml(), dracula_alacritty);
        }

        #[test]
        fn convert_iterm_toml() {
            let dracula_iterm = read_fixture("tests/fixtures/Dracula.itermcolors");
            let dracula_alacritty_toml: String = "# Default colors
[colors.primary]
background = '0x1e1f28'
foreground = '0xf8f8f2'

# Cursor colors
[colors.cursor]
text =   '0xffffff'
cursor = '0xbbbbbb'

# Normal colors
[colors.normal]
black =   '0x000000'
red =     '0xff5555'
green =   '0x50fa7b'
yellow =  '0xf1fa8c'
blue =    '0xbd93f9'
magenta = '0xff79c6'
cyan =    '0x8be9fd'
white =   '0xbbbbbb'

# Bright colors
[colors.bright]
black =   '0x555555'
red =     '0xff5555'
green =   '0x50fa7b'
yellow =  '0xf1fa8c'
blue =    '0xbd93f9'
magenta = '0xff79c6'
cyan =    '0x8be9fd'
white =   '0xffffff'
"
            .to_string();
            let scheme = ColorScheme::from_iterm(&dracula_iterm).unwrap();
            assert_eq!(scheme.to_toml(), dracula_alacritty_toml);
        }
    }
}
