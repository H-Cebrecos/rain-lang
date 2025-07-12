-- Begin Component Declaration
component counter
generic
(
	start	: integer := 0,
	end_val	: integer := 200,
	duration	: integer
);
port
(
	clk	: In  std_logic,
	reset	: In  std_logic,
	signal	: Out std_logic_vector(31 downto 0),
	count	: Out std_logic_vector(18 downto 0)
);
end component;
-- End Component Declaration

-- Begin Instantiation Template
instance_name : counter
generic map
(
	start	=> ,
	end_val	=> ,
	duration	=> 
);
port map
(
	clk	=> ,
	reset	=> ,
	signal	=> ,
	count	=> 
);
-- End Instantiation Template

