use crate::chunk_type::ChunkType;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::clap::AppSettings;
use structopt::StructOpt;

/*
pngme encode ./dice.png ruSt "This is a secret message!
pngme decode ./dice.png ruSt
pngme remove ./dice.png ruSt
pngme print ./dice.png
*/

#[derive(StructOpt)]
#[structopt(global_settings(&[AppSettings::VersionlessSubcommands]))]
pub(crate) struct Cli {
    #[structopt(subcommand)]
    pub(crate) subcommand: Subcommand,
}

#[derive(StructOpt, Debug, PartialEq)]
pub(crate) struct EncodeArgs {
    #[structopt(parse(from_os_str), help = "Path to the input PNG")]
    pub(crate) input_file_path: PathBuf,
    #[structopt(
            parse(try_from_str = ChunkType::from_str),
            help = "Chunk type (like 'ruSt')"
        )]
    pub(crate) chunk_type: ChunkType,
    #[structopt(help = "Your secret message")]
    pub(crate) message: String,
    #[structopt(parse(from_os_str), help = "Path to the output PNG (optional)")]
    pub(crate) output_file_path: Option<PathBuf>,
}

#[derive(StructOpt, Debug, PartialEq)]
pub(crate) struct DecodeArgs {
    #[structopt(parse(from_os_str), help = "Path to the PNG")]
    pub(crate) file_path: PathBuf,
    #[structopt(
            parse(try_from_str = ChunkType::from_str),
            help = "Chunk type (like 'ruSt')"
        )]
    pub(crate) chunk_type: ChunkType,
}

#[derive(StructOpt, Debug, PartialEq)]
pub(crate) struct RemoveArgs {
    #[structopt(parse(from_os_str), help = "Path to the PNG")]
    pub(crate) file_path: PathBuf,
    #[structopt(
            parse(try_from_str = ChunkType::from_str),
            help = "Chunk type (like 'ruSt')"
        )]
    pub(crate) chunk_type: ChunkType,
}

#[derive(StructOpt, Debug, PartialEq)]
pub(crate) struct PrintArgs {
    #[structopt(parse(from_os_str), help = "Path to the PNG")]
    pub(crate) file_path: PathBuf,
}

#[derive(Debug, StructOpt, PartialEq)]
pub(crate) enum Subcommand {
    #[structopt(about = "Add a secret message to a PNG")]
    Encode(EncodeArgs),
    #[structopt(about = "Show the secret message in a PNG")]
    Decode(DecodeArgs),
    #[structopt(about = "Remove a secret message from a PNG")]
    Remove(RemoveArgs),
    #[structopt(about = "Print every chunk in a PNG")]
    Print(PrintArgs),
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub(crate) fn test_encode() {
        let expected = Subcommand::Encode(EncodeArgs {
            input_file_path: PathBuf::from("/a/b/c"),
            chunk_type: ChunkType::from_str("RuSt").unwrap(),
            message: "Secret decoder ring".to_string(),
            output_file_path: None,
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "encode",
            "/a/b/c",
            "RuSt",
            "Secret decoder ring",
        ]);
        let actual = cli.subcommand;

        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_encode_with_output_file() {
        let expected = Subcommand::Encode(EncodeArgs {
            input_file_path: PathBuf::from("/a/b/c"),
            chunk_type: ChunkType::from_str("RuSt").unwrap(),
            message: "Secret decoder ring".to_string(),
            output_file_path: Some(PathBuf::from("/output/file/path")),
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "encode",
            "/a/b/c",
            "RuSt",
            "Secret decoder ring",
            "/output/file/path",
        ]);
        let actual = cli.subcommand;

        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_decode() {
        let expected = Subcommand::Decode(DecodeArgs {
            file_path: PathBuf::from("/a/b/c"),
            chunk_type: ChunkType::from_str("PnGm").unwrap(),
        });
        let cli = Cli::from_iter(vec!["pngme", "decode", "/a/b/c", "PnGm"]);
        let actual = cli.subcommand;

        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_remove() {
        let expected = Subcommand::Remove(RemoveArgs {
            file_path: PathBuf::from("/a/b/c"),
            chunk_type: ChunkType::from_str("imAG").unwrap(),
        });
        let cli = Cli::from_iter(vec!["pngme", "remove", "/a/b/c", "imAG"]);
        let actual = cli.subcommand;

        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_print() {
        let expected = Subcommand::Print(PrintArgs {
            file_path: PathBuf::from("/a/b/c"),
        });
        let cli = Cli::from_iter(vec!["pngme", "print", "/a/b/c"]);
        let actual = cli.subcommand;

        assert_eq!(expected, actual);
    }

    #[test]
    pub(crate) fn test_unknown_subcommand() {
        let result = Cli::from_iter_safe(vec!["pngme", "blah-blah", "some-argument"]);

        assert!(result.is_err());
    }
}