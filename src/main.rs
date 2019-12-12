#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error in swayipc-rs: {0}")]
    SwayIPC(swayipc::Error),
    #[error("no input with active layout found")]
    InputNotFound,
}

fn main() -> Result<(), Error> {
    swayipc::Connection::new()
        .map_err(Error::SwayIPC)?
        .get_inputs()
        .map_err(Error::SwayIPC)?
        .iter()
        .find_map(|input| input.xkb_active_layout_name.as_ref())
        .map(|layout| println!("{}", layout))
        .ok_or(Error::InputNotFound)
}
