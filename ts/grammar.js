module.exports = grammar({
  name: 'Clue',
  rules: {
    expr: $ => choice($.block, $.number, $.local_decl, $.if_stmt, $.match_stmt, $.try_stmt),

    exprlist: $ => repeat1($.expr),

    number: $ => /\d+(\.\d+)*/,

    block: $ => seq('{', optional($.exprlist), '}'),

    local_decl: $=> seq('local', /[a-zA-Z_]+[a-zA-Z_0-9]*/, '=', $.expr),

    if_stmt: $ => seq($.if_block, repeat($.elseif_block), optional($.else_block)),
    if_block: $ => seq('if', $.expr, $.block),
    elseif_block: $ => seq('elseif', $.expr, $.block),
    else_block: $ => seq('else', $.block),

    match_stmt: $ => seq('match', $.expr, $.match_block),
    match_block: $ => seq('{', repeat1($.match_case), '}'),
    match_case: $ => seq($.match_expr, '=>', $.block),
    match_expr: $ => choice('default', $.expr),

    try_stmt: $ => seq($.try_block, optional($.catch_block)),
    try_block: $ => seq('try', $.expr, $.block),
    catch_block: $ => seq('catch', optional(/[a-zA-Z_]+[a-zA-Z_0-9]*/), $.block),

  }
});
