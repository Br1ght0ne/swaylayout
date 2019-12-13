use swayipc::{
    reply::{Event, InputChange, InputEvent},
    Connection, EventType,
};

#[derive(Debug, structopt::StructOpt)]
struct Args {
    /// Input device identifier
    identifier: String,
    /// Listen to input change events and continuously output the new layout
    #[structopt(short, long)]
    listen: bool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("error in swayipc-rs: {0}")]
    SwayIPC(swayipc::Error),
    #[error("error in paw: {0}")]
    Paw(#[from] std::io::Error),
    #[error("wrong input change event: {0:?}")]
    WrongInputChange(InputChange),
    #[error("no active layout found")]
    LayoutNotFound,
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn new_layout_name(event: InputEvent) -> Result<String> {
    if let InputChange::XkbLayout = event.change {
        event
            .input
            .xkb_active_layout_name
            .ok_or(Error::LayoutNotFound)
    } else {
        Err(Error::WrongInputChange(event.change))
    }
}

fn current_layout_name(conn: &mut Connection, identifier: &str) -> Result<String> {
    conn.get_inputs()
        .map_err(Error::SwayIPC)?
        .into_iter()
        .find(|input| input.identifier == identifier)
        .and_then(|input| input.xkb_active_layout_name)
        .ok_or(Error::LayoutNotFound)
}

fn input_events(conn: Connection) -> Result<impl Iterator<Item = InputEvent>> {
    conn.subscribe(&[EventType::Input])
        .map(|iter| {
            iter.filter_map(|event| match event {
                Ok(Event::Input(input_event)) => Some(input_event),
                _ => None,
            })
        })
        .map_err(Error::SwayIPC)
}

#[paw::main]
fn main(args: Args) -> Result<()> {
    let mut conn = Connection::new().map_err(Error::SwayIPC)?;
    println!("{}", current_layout_name(&mut conn, &args.identifier)?);

    if args.listen {
        let events = input_events(conn)?.filter(|event| event.input.identifier == args.identifier);
        for event in events {
            if let Ok(name) = new_layout_name(event) {
                println!("{}", name);
            }
        }
    }

    Ok(())
}
