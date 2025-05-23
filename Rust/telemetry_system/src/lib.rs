use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub struct RandomTelemetryClient {
    online_status: bool,
    diagnostic_message_result: String,
    connection_events_simulator: StdRng,
}

impl RandomTelemetryClient {
    pub const DIAGNOSTIC_MESSAGE: &'static str = "AT#UD";

    pub fn new() -> Self {
        Self {
            online_status: false,
            diagnostic_message_result: String::new(),
            connection_events_simulator: StdRng::seed_from_u64(42),
        }
    }
}

impl TelemetryClient for RandomTelemetryClient {
    fn get_online_status(&self) -> bool {
        self.online_status
    }

    fn connect(&mut self, telemetry_server_connection_string: &str) {
        if telemetry_server_connection_string.is_empty() {
            panic!("Invalid telemetry server connection string");
        }

        let success = self.connection_events_simulator.gen_range(0..10) <= 8;
        self.online_status = success;
    }

    fn disconnect(&mut self) {
        self.online_status = false;
    }

    fn send(&mut self, message: &str) {
        if message.is_empty() {
            panic!("Invalid message");
        }

        if message == Self::DIAGNOSTIC_MESSAGE {
            self.diagnostic_message_result = "LAST TX rate................ 100 MBPS\r\n\
                HIGHEST TX rate............. 100 MBPS\r\n\
                LAST RX rate................ 100 MBPS\r\n\
                HIGHEST RX rate............. 100 MBPS\r\n\
                BIT RATE.................... 100000000\r\n\
                WORD LEN.................... 16\r\n\
                WORD/FRAME.................. 511\r\n\
                BITS/FRAME.................. 8192\r\n\
                MODULATION TYPE............. PCM/FM\r\n\
                TX Digital Los.............. 0.75\r\n\
                RX Digital Los.............. 0.10\r\n\
                BEP Test.................... -5\r\n\
                Local Rtrn Count............ 00\r\n\
                Remote Rtrn Count........... 00"
                .to_string();
        }
    }

    fn receive(&mut self) -> String {
        if self.diagnostic_message_result.is_empty() {
            let message_length = self.connection_events_simulator.gen_range(60..110);
            let message = (0..message_length)
                .map(|_| {
                    char::from_u32(self.connection_events_simulator.gen_range(0..40) as u32 + 86)
                        .unwrap()
                })
                .collect::<String>();
            message
        } else {
            let message = self.diagnostic_message_result.clone();
            self.diagnostic_message_result.clear();
            message
        }
    }
}

pub trait TelemetryClient {
    fn get_online_status(&self) -> bool;
    fn connect(&mut self, telemetry_server_connection_string: &str);
    fn disconnect(&mut self);
    fn send(&mut self, message: &str);
    fn receive(&mut self) -> String;
}

pub struct TelemetryDiagnosticControls<Client: TelemetryClient> {
    telemetry_client: Client,
    diagnostic_info: String,
}

impl<Client: TelemetryClient> TelemetryDiagnosticControls<Client> {
    pub const DIAGNOSTIC_CHANNEL_CONNECTION_STRING: &'static str = "*111#";

    pub fn new(client: Client) -> Self {
        Self {
            telemetry_client: client,
            diagnostic_info: String::new(),
        }
    }

    pub fn get_diagnostic_info(&self) -> &str {
        &self.diagnostic_info
    }

    pub fn set_diagnostic_info(&mut self, diagnostic_info: String) {
        self.diagnostic_info = diagnostic_info;
    }

    pub fn check_transmission(&mut self) -> Result<(), &'static str> {
        self.diagnostic_info.clear();

        self.telemetry_client.disconnect();

        let mut retry_left = 3;
        while !self.telemetry_client.get_online_status() && retry_left > 0 {
            self.telemetry_client
                .connect(Self::DIAGNOSTIC_CHANNEL_CONNECTION_STRING);
            retry_left -= 1;
        }

        if !self.telemetry_client.get_online_status() {
            return Err("Unable to connect.");
        }

        self.telemetry_client
            .send(RandomTelemetryClient::DIAGNOSTIC_MESSAGE);
        self.diagnostic_info = self.telemetry_client.receive();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_transmission_should_send_a_diagnostic_message_and_receive_a_status_message_response(
    ) {
        let mut telemetry_diagnostic_controls =
            TelemetryDiagnosticControls::new(RandomTelemetryClient::new());

        telemetry_diagnostic_controls.check_transmission().unwrap();

        assert!(!telemetry_diagnostic_controls
            .get_diagnostic_info()
            .is_empty());
    }

    #[test]
    fn fails_when_client_cannot_connect() {
        let mut controls = TelemetryDiagnosticControls::new(AlwaysOfflineClient {});
        assert!(controls.check_transmission().is_err())
    }

    struct AlwaysOfflineClient;
    impl TelemetryClient for AlwaysOfflineClient {
        fn get_online_status(&self) -> bool {
            false
        }

        fn connect(&mut self, _telemetry_server_connection_string: &str) {}

        fn disconnect(&mut self) {}

        fn send(&mut self, _message: &str) {
            panic!("this should never be called!")
        }

        fn receive(&mut self) -> String {
            panic!("this should never be called!")
        }
    }
}
