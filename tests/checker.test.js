const os = require("os");

const platform = `${os.platform()}-${os.arch()}`;

const checker = require(`../platforms/mgchecker-${platform}`);
console.log(checker);

describe("matching cities to foods", () => {
  beforeAll(() => {
    return checker.initSensitiveSet(["王八", "王八蛋"]);
  });

  test("has words", (done) => {
    checker.hasSensitiveWord("网吧是王八蛋", (err, r) => {
      expect(r).toBeTruthy();
      done();
    });
  });

  test("has words", () => {
    const r = checker.hasSensitiveWordSync("王八是网吧");
    expect(r).toBeTruthy();
  });

  test("no words", (done) => {
    checker.hasSensitiveWord("他出个小王，你出个大王", (err, r) => {
      expect(r).toBe(false);
      done();
    });
  });

  test("adds 1 + 2 to equal 3", () => {
    expect(sum(1, 2)).toBe(3);
  });
});

function sum(a, b) {
  return a + b;
}
