module.exports = {
    devServer: {
        // This will forward any request that does not match a static file to localhost:8000
        proxy: 'http://localhost:8000'
    }
}