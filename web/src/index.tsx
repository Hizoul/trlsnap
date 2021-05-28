import React from 'react'
import ReactDOM from 'react-dom'
import ReactApp from "./app"
import { data_to_nedb } from "./data"

data_to_nedb()

ReactDOM.render(<ReactApp />, document.getElementById('root'))