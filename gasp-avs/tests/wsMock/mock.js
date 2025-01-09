import {createProxyMiddleware, responseInterceptor} from "http-proxy-middleware";
import  express  from "express";

const wsProxy = createProxyMiddleware({
    target: 'ws://0.0.0.0:9946',
    // pathRewrite: {
    //  '^/websocket' : '/socket',        // rewrite path.
    //  '^/removepath' : ''               // remove path.
    // },
    changeOrigin: true, // for vhosted sites, changes host header to match to target's host
    ws: true, // enable websocket proxy
    logger: console,
});

const app = express();
//app.use('/', express.static(__dirname)); // demo page
app.use(wsProxy); // add the proxy to express

const server = app.listen(3000);

console.log('[DEMO] Server: listening on port 3000');
console.log('[DEMO] Opening: http://localhost:3000');

//require('open')('http://localhost:3000');
