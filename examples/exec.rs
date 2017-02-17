extern crate boondock;

use boondock::{ExecOptions, ContainerListOptions, Docker};

fn main() {
    let docker = Docker::connect_with_defaults().unwrap();
    let c_opts = ContainerListOptions::default();
    let e_opts = ExecOptions::default();
    let cmd = "ls".to_string();
    if let Some(container) = docker.containers(c_opts).unwrap().get(0) {
        docker.exec(container, e_opts, cmd).unwrap();
        println!("{}","toto");
    }
}
