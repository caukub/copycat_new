
/**
 * Let's say on our shared host the assigned (available) ports are
 * are <server port> to <server port + 9>
 */  
function main() {
    const ports = server.ports;
    const server_port = server.port.server;
  
    ports.forEach((port, index) => {
      
      const isValidNumber = typeof port === 'number' && Number.isFinite(port);
  
      if (isValidNumber) {
        if (port < server_port || port > server_port + 9) {
            if (port == 8192) {
                return {
                    message: "you are using default votifier port",
                    description: "you have to configure the port in configuration, see tut:"
                }
            } else if (port == 8123) {
                return {
                    message: "you are using default dynmap",
                    description: "you have to configure the port in configuration, see tut"
                }
            } else if (port == 19132) {
                return {
                    message: "you are using default geysermc port",
                    description: "you have to configure the plugin in configuration, see tutorial:"
                }
            } else {
                return {
                    message: "the port for ${plugin} is out of your assigned ports range",
                    description: "bez do zalozky informace a v ${plugin} zmen port"
                }
            }
        }
      }
    });
  }
  
  main();