#[macro_use]
extern crate maplit;

extern crate rxr;
use rxr::dosbox_config::DosboxConfig;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn to_owned_map(map: HashMap<&str, &str>) -> HashMap<String, String> {
        map.iter()
            .map(|(key, val)| (String::from(*key), String::from(*val)))
            .fold(HashMap::new(), |mut fmap, (key, val)| {
                fmap.insert(key, val);
                fmap
            })
    }

    #[test]
    fn parsing_a_config_file() {
        let config_a = String::from(
            r#" [render]
                # never scale
                scaler=none

                [cpu]
                # Always use dynamic core.
                core=dynamic

                [midi]
                # Port for midi server.
                midiconfig=128:0

                [sblaster]
                irq=7
                dma=1
                hdma=5
                
                [autoexec]
                # No configuration
                "#,
        );

        let config_b = String::from(
            r#" [sblaster]
                # Program is configured for IRQ=5
                irq=5

                [ipx]
                # ipx -- Enable ipx over UDP/IP emulation.
                ipx=false

                [autoexec]
                # Lines in this section will be run at startup.
                @echo off
                mount c "..\game"
                imgmount d "..\game\game.iso" -t iso -fs iso
                c:
                cls
                game.exe
                exit"#,
        );

        let lines_a: Vec<&str> = config_a.split("\n").collect();
        let lines_b: Vec<&str> = config_b.split("\n").collect();

        let config_a = DosboxConfig::parse(lines_a.as_slice()).unwrap();
        let config_b = DosboxConfig::parse(lines_b.as_slice()).unwrap();

        println!("Config A: {:#?}", config_a);
        println!("Config B: {:#?}", config_b);

        let merged = config_a.merge(&config_b);

        let expected = DosboxConfig {
            autoexec: vec![
                "@echo off",
                "mount c \"../game\"",
                "imgmount d \"../game/game.iso\" -t iso -fs iso",
                "c:",
                "cls",
                "game.exe",
                "exit",
            ].iter()
                .map(|s| String::from(*s))
                .collect(),
            settings: hashmap!{
            String::from("render") =>
                        to_owned_map(hashmap!{
                            "scaler" => "none"
                        }),

                        String::from("cpu") =>
                        to_owned_map(hashmap!{
                            "core" => "dynamic"
                        }),

                        String::from("midi") =>
                        to_owned_map(hashmap!{
                            "midiconfig" => "128:0"
                        }),

                        String::from("sblaster") =>
                        to_owned_map(hashmap!{
                            "irq" => "5",
                            "dma" => "1",
                            "hdma" => "5"
                        }),

                        String::from("ipx") =>
                        to_owned_map(hashmap!{
                            "ipx" => "false"
                        }),
                    },
        };

        assert_eq!(merged, expected);

        let config_path = PathBuf::from("merged.conf");
        merged.write(&config_path).unwrap();
        let file = DosboxConfig::read(&config_path).unwrap();

        assert_eq!(merged, file);
        fs::remove_file(&config_path).unwrap();
    }
}
