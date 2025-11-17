[
  (macro_def)
  (function_def)
] @function.around

[
  (bracket_comment)
  (line_comment)
] @comment.inside

(line_comment)+ @comment.around

(bracket_comment) @comment.around
