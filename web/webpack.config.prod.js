let modifyConf = require("./webpack.config")
let webpack = require("webpack")
modifyConf.plugins.push(new webpack.optimize.ModuleConcatenationPlugin())
modifyConf.mode = "production"
module.exports = modifyConf