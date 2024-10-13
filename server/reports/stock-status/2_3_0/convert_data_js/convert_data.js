// passing sql functions into js
// const { sql } = Host.getFunctions();
function convert_data() {
  const res = JSON.parse(Host.inputString());
  res.items.nodes.forEach((r) => {
    // Some data processing
    r.stats.newField = 'test js function';
    r.stats.theNewFieldWeAreAdding = 1;
  });
  Host.outputString(JSON.stringify(res));
}
module.exports = { convert_data };