import { ArchiveReader } from '../mod.ts';

let myZip = new ArchiveReader();

console.log(myZip.open("bruh.tar"));
