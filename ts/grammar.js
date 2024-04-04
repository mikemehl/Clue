module.exports = grammar({
  name: 'Clue',
  rules: {
    expr: $ => choice($.block, $.number, $.local_decl),
    exprlist: $ => repeat1(seq($.expr, '\n')),
    number: $ => /\d+(\.\d+)*/,
    block: $ => seq('{', optional($.expr), '}'),
    local_decl: $=> seq('local', /[a-zA-Z_]+[a-zA-Z_0-9]*/, '=', $.expr)
  }
});
