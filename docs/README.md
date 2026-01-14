# copycat
Features
- YAML/JSON invalid detection and error report

TODO
detekce stacktrace a highlight .jar v ni ale TOTO: ~[Citizens-2.0.41-b4029.jar:?] NE
16:57:33[ERROR] Error loading the plugin, something is not configured right or an exception occured. More detail:
16:57:33[WARN] java.lang.IllegalArgumentException: Invalid key. Must be [a-z0-9/._-]: PROTECTION
16:57:33[WARN] at com.google.common.base.Preconditions.checkArgument(Preconditions.java:220)
16:57:33[WARN] at org.bukkit.NamespacedKey.<init>(NamespacedKey.java:86)
16:57:33[WARN] at org.bukkit.NamespacedKey.minecraft(NamespacedKey.java:179)
16:57:33[WARN]
at ItemsAdder.jar//ia.m.G.b(SourceFile:1424)
16:57:33[WARN] at ItemsAdder.jar//ia.m.G.c(SourceFile:581)
16:57:33[WARN] at org.bukkit.craftbukkit.v1_20_R1.scheduler.CraftTask.run(CraftTask.java:101)
16:57:33[WARN] at org.bukkit.craftbukkit.v1_20_R1.scheduler.CraftScheduler.mainThreadHeartbeat(CraftScheduler.java:480)
16:57:33[WARN] at net.minecraft.server.MinecraftServer.b(MinecraftServer.java:1506)
16:57:33[WARN] at net.minecraft.server.dedicated.DedicatedServer.b(DedicatedServer.java:486)
16:57:33[WARN] at net.minecraft.server.MinecraftServer.a(MinecraftServer.java:1420)
16:57:33[WARN] at net.minecraft.server.MinecraftServer.w(MinecraftServer.java:1191)
16:57:33[WARN] at net.minecraft.server.MinecraftServer.lambda$spin$0(MinecraftServer.java:322)
16:57:33[WARN] at java.base/java.lang.Thread.run(Unknown Source)