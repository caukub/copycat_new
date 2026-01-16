// only forge
if (server.version <= "1.16.5" && server.version > "1.8.9" && java.version != null && java.version > 8 || log.contains_line("")) {
    return {
        header: "Java Version is too new!",
        description: "You must use Java 8 for forge 1.16.5 and lower"
    }
}