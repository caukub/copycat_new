// Let's say we are shared host which server port is always 5 digit long and ends with 0
function main() {
    if (!isValidHostPort(server.port.server)) {
        return {
            message: "This server is not hosted on MyHost.com",
            private: true // This detection doesn't show to the user but is accessible through API
        }
    }
}

main();

/**
 * Checks if a port number is exactly 5 digits and ends with the digit 0.
 * * @param {number} port - The port number to check.
 * @returns {boolean} True if the port is a 5-digit number ending in 0, false otherwise.
 */
function isValidHostPort(port) {
    if (!port.isFinite()) return true; // the port was not found (is not a valid number) so we skip the check

    const str = num.toString();
  
    const isFiveDigits = str.length === 5;
    const endsWithZero = str.endsWith('0');
  
    return str.length === 5 && port.endsWith('0');
}