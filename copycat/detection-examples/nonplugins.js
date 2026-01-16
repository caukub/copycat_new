[15:14:39] [ServerMain/ERROR]: [DirectoryProviderSource] Error loading plugin: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/citizensapi-2.0.33-SNAPSHOT-sources.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!

ugin Remapper Thread - 0/INFO]: [PluginRemapper] Remapping plugin 'plugins/AuthMe-5.7.0-SNAPSHOT-javadoc.jar'...
[21:45:04] [Paper Plugin Remapper Thread - 0/INFO]: [PluginRemapper] Done remapping plugin 'plugins/AuthMe-5.7.0-SNAPSHOT-javadoc.jar' in 206ms.
[21:45:04] [ServerMain/ERROR]: [PluginRemapper] Encountered exception remapping plugins
java.util.concurrent.CompletionException: java.lang.RuntimeException: Failed to open plugin jar plugins/AuthMe-5.7.0-SNAPSHOT-sources.jar
	at java.base/java.util.concurrent.CompletableFuture.reportJoin(Unknown Source) ~[?:?]
	at java.base/java.util.concurrent.CompletableFuture.join(Unknown Source) ~[?:?]
	at io.papermc.paper.pluginremap.PluginRemapper.waitForAll(PluginRemapper.java:414) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.pluginremap.PluginRemapper.rewritePluginDirectory(PluginRemapper.java:210) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.prepareContext(DirectoryProviderSource.java:53) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.prepareContext(DirectoryProviderSource.java:17) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.util.EntrypointUtil.registerProvidersFromSource(EntrypointUtil.java:14) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.PluginInitializerManager.load(PluginInitializerManager.java:113) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at net.minecraft.server.Main.main(Main.java:108) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.PaperBootstrap.boot(PaperBootstrap.java:21) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at org.bukkit.craftbukkit.Main.main(Main.java:234) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paperclip.Paperclip.lambda$main$0(Paperclip.java:42) ~[?:?]
	at java.base/java.lang.Thread.run(Unknown Source) ~[?:?]
Caused by: java.lang.RuntimeException: Failed to open plugin jar plugins/AuthMe-5.7.0-SNAPSHOT-sources.jar
	at io.papermc.paper.pluginremap.PluginRemapper.remap(PluginRemapper.java:343) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.pluginremap.PluginRemapper.remapPlugin(PluginRemapper.java:270) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.pluginremap.PluginRemapper.rewritePluginDirectory(PluginRemapper.java:208) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	... 9 more
Caused by: java.util.zip.ZipException: ZIP file can't be opened as a file system because entry "/." has a '.' or '..' element in its name
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystem.initCEN(Unknown Source) ~[jdk.zipfs:?]
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystem.<init>(Unknown Source) ~[jdk.zipfs:?]
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystemProvider.getZipFileSystem(Unknown Source) ~[jdk.zipfs:?]
	at jdk.zipfs/jdk.nio.zipfs.ZipFileSystemProvider.newFileSystem(Unknown Source) ~[jdk.zipfs:?]
	at java.base/java.nio.file.FileSystems.newFileSystem(Unknown Source) ~[?:?]
	at java.base/java.nio.file.FileSystems.newFileSystem(Unknown Source) ~[?:?]
	at io.papermc.paper.pluginremap.PluginRemapper.remap(PluginRemapper.java:294) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	... 11 more
[21:45:04] [ServerMain/ERROR]: [DirectoryProviderSource] Error loading plugin: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/AuthMe-5.7.0-SNAPSHOT-javadoc.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
java.lang.RuntimeException: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/AuthMe-5.7.0-SNAPSHOT-javadoc.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
	at io.papermc.paper.plugin.provider.source.FileProviderSource.registerProviders(FileProviderSource.java:85) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.registerProviders(DirectoryProviderSource.java:63) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.provider.source.DirectoryProviderSource.registerProviders(DirectoryProviderSource.java:17) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.util.EntrypointUtil.registerProvidersFromSource(EntrypointUtil.java:15) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.plugin.PluginInitializerManager.load(PluginInitializerManager.java:113) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at net.minecraft.server.Main.main(Main.java:108) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paper.PaperBootstrap.boot(PaperBootstrap.java:21) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at org.bukkit.craftbukkit.Main.main(Main.java:234) ~[paper-1.21.11.jar:1.21.11-92-e7445e1]
	at io.papermc.paperclip.Paperclip.lambda$main$0(Paperclip.java:42) ~[?:?]
	at java.base/java.lang.Thread.run(Unknown Source) ~[?:?]
Caused by: java.lang.IllegalArgumentException: Directory 'plugins/.paper-remapped/AuthMe-5.7.0-SNAPSHOT-javadoc.jar' does not contain a paper-plugin.yml or plugin.yml! Could not determine plugin type, cannot load a plugin from it!
	... 10 more
[21:45:04] [ServerMain/INFO]: [PluginInitializerManager] Initialized 1 plugin