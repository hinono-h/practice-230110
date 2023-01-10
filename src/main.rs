use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::core::{formats, meta};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("Not Found any files.");

    let file = std::fs::File::open(&path).expect("Cannot open a file.");

    let media_src_stream = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let meta_options: meta::MetadataOptions = Default::default();
    let fmt_options: formats::FormatOptions = Default::default();

    let probe = symphonia::default::get_probe()
        .format(&hint, media_src_stream, &fmt_options, &meta_options)
        .expect("Unsupported format error.");

    let mut format = probe.format;
}
