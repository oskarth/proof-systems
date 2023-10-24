use clap::{arg, value_parser, Arg, ArgAction, Command};
use kimchi_optimism::cannon::VmConfiguration;
use std::process::ExitCode;

fn cli() -> VmConfiguration {
    use kimchi_optimism::cannon::*;

    let app_name = "zkvm";
    let cli = Command::new(app_name)
        .version("0.1")
        .about("MIPS-based zkvm")
        .arg(arg!(--input <FILE> "initial state file").default_value("state.json"))
        .arg(arg!(--output <FILE> "output state file").default_value("out.json"))
        .arg(arg!(--meta <FILE> "metadata file").default_value("meta.json"))
        // The CLI arguments below this line are ignored at this point
        .arg(
            Arg::new("proof-at")
                .short('p')
                .long("proof-at")
                .value_name("FREQ")
                .default_value("never")
                .value_parser(step_frequency_parser),
        )
        .arg(
            Arg::new("proof-fmt")
                .long("proof-fmt")
                .value_name("FORMAT")
                .default_value("proof-%d.json"),
        )
        .arg(
            Arg::new("snapshot-fmt")
                .long("snapshot-fmt")
                .value_name("FORMAT")
                .default_value("state-%d.json"),
        )
        .arg(
            Arg::new("stop-at")
                .long("stop-at")
                .value_name("FREQ")
                .default_value("never")
                .value_parser(step_frequency_parser),
        )
        .arg(
            Arg::new("info-at")
                .long("info-at")
                .value_name("FREQ")
                .default_value("never")
                .value_parser(step_frequency_parser),
        )
        .arg(
            Arg::new("pprof-cpu")
                .long("pprof-cpu")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(host: [HOST] "host program specification <host program> [host program arguments]")
                .num_args(1..)
                .last(true)
                .value_parser(value_parser!(String)),
        );

    let cli = cli.get_matches();

    let input_state_file = cli
        .get_one::<String>("input")
        .expect("Default ensures there is always a value");

    let output_state_file = cli
        .get_one::<String>("output")
        .expect("Default ensures there is always a value");

    let metadata_file = cli
        .get_one::<String>("meta")
        .expect("Default ensures there is always a value");

    let proof_at = cli.get_one::<StepFrequency>("proof-at").expect("");
    let info_at = cli.get_one::<StepFrequency>("info-at").expect("");
    let stop_at = cli.get_one::<StepFrequency>("stop-at").expect("");

    let proof_fmt = cli.get_one::<String>("proof-fmt").expect("");
    let snapshot_fmt = cli.get_one::<String>("snapshot-fmt").expect("");
    let pprof_cpu = cli.get_one::<bool>("pprof-cpu").expect("");

    let host_spec = cli
        .get_many::<String>("host")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default();

    let host = if host_spec.is_empty() {
        None
    } else {
        Some(HostProgram {
            name: host_spec[0].to_string(),
            arguments: host_spec[1..]
                .to_vec()
                .iter()
                .map(|x| x.to_string())
                .collect(),
        })
    };

    VmConfiguration {
        input_state_file: input_state_file.to_string(),
        output_state_file: output_state_file.to_string(),
        metadata_file: metadata_file.to_string(),
        proof_at: proof_at.clone(),
        stop_at: stop_at.clone(),
        info_at: info_at.clone(),
        proof_fmt: proof_fmt.to_string(),
        snapshot_fmt: snapshot_fmt.to_string(),
        pprof_cpu: *pprof_cpu,
        host,
    }
}

pub fn main() -> ExitCode {
    let configuration = cli();

    println!("configuration\n{:#?}", configuration);

    // TODO: Logic
    ExitCode::FAILURE
}
