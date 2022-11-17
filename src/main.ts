import * as variable from './variable';
import * as enums from './enums';
import * as strings from './strings';
import * as options from './options';
import * as arrays from './arrays';
import * as results from './results';
import * as structs from './structs';
import * as traits from './traits';
import * as lifetime from './lifetime';
import * as boxs from './boxs';
import * as rcs from './rcs';

function main() {
  variable.variable();
  enums.enums();
  strings.strings();
  options.options();
  arrays.arrays();
  results.results();
  structs.structs();
  traits.traits();
  lifetime.lifetime();
  boxs.boxs();
  rcs.rcs();
}

main();
