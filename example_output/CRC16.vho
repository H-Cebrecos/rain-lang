-- Begin Component Declaration
component CRC16
port
(
	clk	: In  std_logic,
	rst	: In  std_logic,
	data	: In  std_logic_vector(7 downto 0),
	valid	: In  std_logic,
	crc	: Out std_logic_vector(15 downto 0)
);
end component;
-- End Component Declaration

-- Begin Instantiation Template
instance_name : CRC16
port map
(
	clk	=> ,
	rst	=> ,
	data	=> ,
	valid	=> ,
	crc	=> 
);
-- End Instantiation Template

