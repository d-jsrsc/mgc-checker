const os = require("os");

const platform = `${os.platform()}-${os.arch()}`;

const checker = require(`../platforms/mgchecker-${platform}`);
console.log(checker);

describe("matching cities to foods", () => {
  beforeAll(() => {
    return checker.init_sensitive_set(["王八", "王八蛋"]);
  });

  test("has words", () => {
    const r = checker.has_sensitive_word("网吧是王八蛋");
    expect(r).toBeTruthy();
  });

  test("has words", () => {
    const r = checker.has_sensitive_word("王八是网吧");
    expect(r).toBeTruthy();
  });

  test("no words", () => {
    const r = checker.has_sensitive_word("他出个小王，你出个大王");
    expect(r).toBe(false);
  });

  test("adds 1 + 2 to equal 3", () => {
    expect(sum(1, 2)).toBe(3);
  });
});

function sum(a, b) {
  return a + b;
}
