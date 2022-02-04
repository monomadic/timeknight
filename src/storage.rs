use crate::state::*;

pub(crate) fn load_state() -> Result<App, crate::Error> {
    let mut path = dirs::home_dir().expect("could not find $HOME directory");
    path.push(".timeknight");
    // if no state can be found, create default

    if path.exists() {
        let data = std::fs::read_to_string(path)?;
        let tasks = ron::from_str(&data)?;
        Ok(App {
            tasks,
            ..Default::default()
        })
    } else {
        Ok(App::default())
    }
}

pub(crate) fn save_state(app: &App) -> Result<(), crate::Error> {
    let mut path = dirs::home_dir().expect("could not find $HOME directory");
    path.push(".timeknight");

    std::fs::write(path, ron::to_string(&app.tasks)?)?;
    Ok(())
}
