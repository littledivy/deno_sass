import Archiver from "../mod.ts";

let tar = new Archiver("bruh.tar");

tar.add("./mod.ts");

tar.archive();
