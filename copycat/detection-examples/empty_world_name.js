if (log.contains_line('Preparing level ""')) {
    return {
        header: "The world is missing name",
        description: "The world name cannot be empty, otherwise the server crashes, set the world name in `server.properties` at `level-name`",
        priority: "5"
    }
}