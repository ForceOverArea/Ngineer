// Requires
const express   = require('express');
const fs        = require('node:fs');
const path      = require('path');

/**
 * Serves content statically in the given app from 
 * the directory at `staticContentDir`
 * @param {Express} expressAppInstance 
 * @param {String} staticContentDir 
 */
function serveStatic(expressAppInstance, staticContentDir)
{
    expressAppInstance.use(
        `/${staticContentDir}`, 
        express.static(path.join(__dirname, staticContentDir))
    );
}

// Create app instance
const app = express();

// Allow static serving from these folders
serveStatic(app, 'ngineer_js/target');
serveStatic(app, 'ngineer_js');
serveStatic(app, '');

// Serve content at root
app.get('/', (req, res) => { 
    const html = fs.readFileSync('index.html', 'utf8');
    res.send(html); 
});

// Start server
app.listen(3000, () => console.log('Ngineer is listening on port 3000.'));