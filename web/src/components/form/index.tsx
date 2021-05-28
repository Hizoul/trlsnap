import { ComponentRegistry } from "@xpfw/form"
import TextField from "./text"

ComponentRegistry.registerComponent("string", TextField)
ComponentRegistry.registerComponent("text", TextField)
ComponentRegistry.registerComponent("select", TextField)