use {
    clap::ArgMatches,
    crate::config::Config,
    std::{ error::Error, path::PathBuf },
};

pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let cfg_file = PathBuf::from("Config.ron");
    if !cfg_file.exists() {
        if matches.is_present("wizard") {
            Config::wizard()?.save()?;
        } else {
            Config::default().save()?;
        }
    };
    Ok(())
}
