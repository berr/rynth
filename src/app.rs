use cpal::traits::{DeviceTrait, HostTrait};

#[derive(Debug)]
struct Opt {
    #[cfg(all(
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
    feature = "jack"
    ))]
    jack: bool,

    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let app = clap::App::new("beep").arg_from_usage("[DEVICE] 'The audio device to use'");
        #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
        ))]
            let app = app.arg_from_usage("-j, --jack 'Use the JACK host");
        let matches = app.get_matches();
        let device = matches.value_of("DEVICE").unwrap_or("pulse").to_string();

        #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
        ))]
        return Opt {
            jack: matches.is_present("jack"),
            device,
        };

        #[cfg(any(
        not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
        not(feature = "jack")
        ))]
        Opt { device }
    }
}

pub fn configure_device() -> anyhow::Result<cpal::Device> {
    let opt = Opt::from_args();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
    any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
    feature = "jack"
    ))]
        // Manually check for flags. Can be passed through cargo with -- e.g.
        // cargo run --release --example beep --features jack -- --jack
        let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
    not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
    not(feature = "jack")
    ))]
        let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
        .expect("failed to find output device");

    println!("Output device: {}", device.name()?);
    Ok(device)
}