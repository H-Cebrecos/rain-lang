entity CRC16 is
port
(
	clk	: In  std_logic,
	rst	: In  std_logic,
	data	: In  std_logic_vector(7 downto 0),
	valid	: In  std_logic,
	crc	: Out std_logic_vector(15 downto 0)
);
end CRC16;

architecture implementation of CRC16 is
	-- declarations
	signal 	partial_1	:  std_logic_vector(15 downto 0);
	signal 	partial_2	:  std_logic_vector(15 downto 0);
	signal 	partial_3	:  std_logic_vector(15 downto 0);
	signal 	partial_4	:  std_logic_vector(15 downto 0);
	signal 	reg	:  std_logic_vector(15 downto 0);
begin
	-- contents
end implementation;

