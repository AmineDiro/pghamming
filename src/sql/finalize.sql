CREATE OPERATOR CLASS binary_op DEFAULT
	FOR TYPE bytea USING ivf AS
	OPERATOR 1 <#> (bytea, bytea) FOR ORDER BY float_ops;