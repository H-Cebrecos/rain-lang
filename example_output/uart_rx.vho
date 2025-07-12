-- Begin Component Declaration
component uart_rx
generic
(
	G_DATA_BITS	: natural := 8,
	G_PARITY	: parity_none,
	G_STOP_BITS	: natural_range_1_to_2
);
port
(
	clk	: In  std_logic,
	rst	: In  std_logic,
	rx	: In  std_logic,
	data	: Out std_logic_vector(7 downto 0),
	valid	: Out std_logic,
	frame_err	: Out std_logic,
	parity_err	: Out std_logic
);
end component;
-- End Component Declaration

-- Begin Instantiation Template
instance_name : uart_rx
generic map
(
	G_DATA_BITS	=> ,
	G_PARITY	=> ,
	G_STOP_BITS	=> 
);
port map
(
	clk	=> ,
	rst	=> ,
	rx	=> ,
	data	=> ,
	valid	=> ,
	frame_err	=> ,
	parity_err	=> 
);
-- End Instantiation Template

