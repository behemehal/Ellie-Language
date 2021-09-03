/*

    NOT READY

*/

pub enum WsMode {
    VisualizeMode,
    Debugger,
    Profiler,
}

pub struct WS {
    pub mode: WsMode,
}

impl WS {
    pub fn start_server() {
        let port = if let Some(port_pos) = env::args().position(|x| x == "--port" || x == "-p") {
            if env::args().len() > port_pos + 1 {
                let port_vec: Vec<String> = env::args().skip(port_pos + 1).collect(); //.nth(code_pos).unwrap();
                let port_string = ellie_lang::cli_utils::clean_up_escape(port_vec.join(" "));

                if let Ok(port) = port_string.clone().parse::<isize>() {
                    port
                } else {
                    println!("{}[Error]{}: Failed to get open port from parameters, supplied parameter ({}{}{}) is not a digit. -h to learn more",
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    port_string,
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                );
                    -1
                }
            } else {
                println!("{}[Error]{}: Failed to get open port from parameters, no data supplied. -h to learn more",
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Red
                ),
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Reset
                ),
            );
                -1
            }
        } else {
            9978
        };

        match Server::bind(format!("127.0.0.1:{}", port)) {
            Ok(server) => {
                println!(
                    "{}[Success]{}: Connect {}{}{} to visualize code exec",
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Green
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Cyan
                    ),
                    format!(
                        "file://{}?s=127.0.0.1:{}",
                        Path::new("../panel/panel.html")
                            .absolutize()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        port
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
                    ),
                );
                for connection in server.filter_map(Result::ok) {
                    thread::spawn(|| {
                        let client = connection.accept().unwrap();
                        println!(
                            "{}[Connection]{}: Connection from {}{}{}, waiting for go signal",
                            ellie_lang::terminal_colors::get_color(
                                ellie_lang::terminal_colors::Colors::Cyan
                            ),
                            ellie_lang::terminal_colors::get_color(
                                ellie_lang::terminal_colors::Colors::Reset
                            ),
                            ellie_lang::terminal_colors::get_color(
                                ellie_lang::terminal_colors::Colors::Yellow
                            ),
                            client.local_addr().unwrap(),
                            ellie_lang::terminal_colors::get_color(
                                ellie_lang::terminal_colors::Colors::Reset
                            ),
                        );
                        let (mut _receiver, mut _sender) = client.split().unwrap();
                    });
                }
            }
            Err(e) => {
                println!("{}[Error: {}]{}: Failed to open port at {}{}{}, use --port or -p to alternate port.",
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Red
                ),
                e.to_string(),
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Reset
                ),
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Red
                ),
                port.to_string(),
                ellie_lang::terminal_colors::get_color(
                    ellie_lang::terminal_colors::Colors::Reset
                ),
            );
            }
        }
    }
}
