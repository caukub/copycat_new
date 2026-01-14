use askama::Template;
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};

use crate::HtmlTemplate;

#[derive(Template)]
#[template(path = "paste/log.html")]
struct PasteTemplate {
    id: String,
    data: LogData,
}

pub struct LogData {
    pub version: Option<String>,
    pub platform: String,
    pub detections: Vec<Detection>,
    pub content: String,
}

pub struct Detection {
    pub header: String,
    pub detail: Option<String>,
    pub solutions: Option<Solutions>,
    pub line: usize,
}

pub struct Solutions {
    format: SolutionFormat,
    lines: Vec<String>,
}

#[derive(Default)]
pub enum SolutionFormat {
    Or,
    OrderedList,
    #[default]
    Raw,
}

pub async fn paste_handler(Path(id): Path<String>) -> Response {
    let format = SolutionFormat::OrderedList;
    let lines = vec![
        "Vypněte server".to_string(),
        "Otevřete Správce Souborů/FTP".to_string(),
        "Smažte složku world".to_string(),
    ];

    let solutions = Solutions {
        format,
        lines: lines.clone(),
    };

    let solutionss = Solutions {
        format: SolutionFormat::default(),
        lines: vec!["V `server.properties` nastavte `level-name`".to_string()],
    };

    let template = PasteTemplate {
        id,
        data: LogData {
            version: Some("1.20.1".to_string()),
            platform: "Paper".to_string(),
            detections: vec![
                Detection {
                    header: "Zjištěn zastaralý plugin".to_string(),
                    detail: None,
                    solutions: Some(solutions),
                    line: 1,
                },
                Detection {
                    header: "Svět nemá jméno".to_string(),
                    detail: None,
                    solutions: Some(solutionss),
                    line: 3
                }
            ],
            content: r#"<span class="i p" id="L1">16:57:33 [INFO] Preparing level ""</span><span class="i p" id="L2">[00:00:00] [Server thread/INFO]: Starting minecraft server...</span><span class="p e">16:57:33[ERROR] Error loading the plugin, something is not configured right or an exception occured. More detail:
            16:57:33[WARN] java.lang.IllegalArgumentException: Invalid key. Must be [a-z0-9/._-]: PROTECTION
            16:57:33[WARN] at com.google.common.base.Preconditions.checkArgument(Preconditions.java:220)
            16:57:33[WARN] at org.bukkit.NamespacedKey.<init>(NamespacedKey.java:86)
            16:57:33[WARN] at org.bukkit.NamespacedKey.minecraft(NamespacedKey.java:179)
            16:57:33[WARN] at <span class="highlight">ItemsAdder.jar</span>//ia.m.G.b(SourceFile:1424)
            16:57:33[WARN] at <span class="highlight">ItemsAdder.jar</span>//ia.m.G.c(SourceFile:581)
            16:57:33[WARN] at org.bukkit.craftbukkit.v1_20_R1.scheduler.CraftTask.run(CraftTask.java:101)
            16:57:33[WARN] at org.bukkit.craftbukkit.v1_20_R1.scheduler.CraftScheduler.mainThreadHeartbeat(CraftScheduler.java:480)
            16:57:33[WARN] at net.minecraft.server.MinecraftServer.b(MinecraftServer.java:1506)
            16:57:33[WARN] at net.minecraft.server.dedicated.DedicatedServer.b(DedicatedServer.java:486)
            16:57:33[WARN] at net.minecraft.server.MinecraftServer.a(MinecraftServer.java:1420)
            16:57:33[WARN] at net.minecraft.server.MinecraftServer.w(MinecraftServer.java:1191)
            16:57:33[WARN] at net.minecraft.server.MinecraftServer.lambda$spin$0(MinecraftServer.java:322)
            16:57:33[WARN] at java.base/java.lang.Thread.run(Unknown Source)</span><span class="w p" id="L3">csssss</span><span class="e p">[14:17:38] [Server thread/ERROR]: Error occurred while enabling Citizens v2.0.41-SNAPSHOT (build 4029) (Is it up to date?)
            java.lang.NoSuchFieldError: Class org.bukkit.entity.EntityType does not have member field 'org.bukkit.entity.EntityType HAPPY_GHAST'
                at <span class="highlight">Citizens-2.0.41-b4029.jar</span>/net.citizensnpcs.nms.v1_21_R5.util.NMSImpl.<clinit>(NMSImpl.java:2698) ~[Citizens-2.0.41-b4029.jar:?]
                at java.base/java.lang.Class.forName0(Native Method) ~[?:?]
                at java.base/java.lang.Class.forName(Unknown Source) ~[?:?]
                at java.base/java.lang.Class.forName(Unknown Source) ~[?:?]
                at io.papermc.reflectionrewriter.runtime.AbstractDefaultRulesReflectionProxy.forName(AbstractDefaultRulesReflectionProxy.java:68) ~[reflection-rewriter-runtime-0.0.3.jar:?]
                at io.papermc.paper.pluginremap.reflect.PaperReflectionHolder.forName(Unknown Source) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at <span class="highlight">Citizens-2.0.41-b4029.jar</span>/net.citizensnpcs.util.NMS.loadBridge(NMS.java:787) ~[Citizens-2.0.41-b4029.jar:?]
                at <span class="highlight">Citizens-2.0.41-b4029.jar</span>/net.citizensnpcs.Citizens.onEnable(Citizens.java:420) ~[Citizens-2.0.41-b4029.jar:?]
                at org.bukkit.plugin.java.JavaPlugin.setEnabled(JavaPlugin.java:280) ~[paper-api-1.21.4-R0.1-SNAPSHOT.jar:?]
                at io.papermc.paper.plugin.manager.PaperPluginInstanceManager.enablePlugin(PaperPluginInstanceManager.java:202) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at io.papermc.paper.plugin.manager.PaperPluginManagerImpl.enablePlugin(PaperPluginManagerImpl.java:109) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at org.bukkit.plugin.SimplePluginManager.enablePlugin(SimplePluginManager.java:520) ~[paper-api-1.21.4-R0.1-SNAPSHOT.jar:?]
                at org.bukkit.craftbukkit.CraftServer.enablePlugin(CraftServer.java:658) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at org.bukkit.craftbukkit.CraftServer.enablePlugins(CraftServer.java:607) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at net.minecraft.server.MinecraftServer.loadWorld0(MinecraftServer.java:743) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at net.minecraft.server.MinecraftServer.loadLevel(MinecraftServer.java:488) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at net.minecraft.server.dedicated.DedicatedServer.initServer(DedicatedServer.java:322) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at net.minecraft.server.MinecraftServer.runServer(MinecraftServer.java:1163) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at net.minecraft.server.MinecraftServer.lambda$spin$2(MinecraftServer.java:310) ~[paper-1.21.4.jar:1.21.4-232-12d8fe0]
                at java.base/java.lang.Thread.run(Unknown Source) ~[?:?]</span>"#.to_string(),
        },
    };

    HtmlTemplate(template).into_response()
}
