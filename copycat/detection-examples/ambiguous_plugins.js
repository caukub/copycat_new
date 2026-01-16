/**
 * Simulace funkce pro vyhledávání v logu
 * @param {string} pattern - Textový vzor k vyhledání
 * @param {string} errorKey - Klíč chyby, který se má vrátit
 * @param {function} callback - Funkce volaná po dokončení (err, result)
 */
function hasLine(pattern, errorKey, callback) {
    const found = true; 
  
    if (found) {
      const result = {
        key: errorKey,
        header: "There are 2 files for plugin {0}",
        solutions: ["Delete {1}", "Delete file {2}"],
        level: "warn",
        detail: "W001"
      };
      return callback(null, result);
    }
  
    callback(null, null);
  }
  
  // --- Použití ---
  
  const pattern = "Ambiguous plugin name '{0}' for files 'plugins/{1}' and 'plugins/{2}' in 'plugins'";
  
  hasLine(pattern, "AmbiguousPluginName", (err, result) => {
    if (err) {
      console.error("Chyba při kontrole logu:", err);
      return;
    }
  
    if (result !== null) {
      // Toto odpovídá tvému 'return result'
      console.log("Nalezena shoda:", result);
      return result;
    }
  });