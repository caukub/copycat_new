if (log.contains_line('Preparing level "')) {
    return {
        header: "The world has no name",
        description: "The world name cannot be empty, set the world name in `server.properties` at `level-name`",
        priority: "5"
    }
}