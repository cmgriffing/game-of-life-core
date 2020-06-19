const childProcess = require("child_process");
const argv = require("yargs").argv;

if (argv.submission) {
  console.log("Plunder more riffiwobbles!");
  process.stdout = childProcess.spawnSync(
    `echo ${argv.submission}`
    // `./target/release/game-of-life-core ${argv.submission}`
  );
  process.exit();
} else {
  process.exit(-1);
}
