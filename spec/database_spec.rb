describe 'database' do
    def run_script(commands)
        raw_output = nil
        IO.popen("./target/debug/db", "r+") do |pipe|
            commands.each do |command|
                pipe.puts command
            end

            pipe.close_write

            raw_output = pipe.gets(nil)
        end
        raw_output.split("\n")
    end

    it 'inserts and retrieves a row' do
        result = run_script([
            "insert 1 user1 person1@example.com",
            "select",
            ".exit",
        ])
        expect(result).to match_array([
            "Executed.",
            "(1, user1, person1@example.com)",
            "Executed.",
        ])
    end

    it 'allows inserting long strings' do
        long_username = "a"*32
        long_email = "a"*255
        script = [
            "insert 1 #{long_username} #{long_email}",
            "select",
            ".exit"
        ]
        result = run_script(script)
        expect(result).to match_array([
            "Executed.",
            "(1, #{long_username}, #{long_email})",
            "Executed.",
        ])
    end

    it 'prints an error message if id is negative' do
        script = [
            "insert -1 angga foo@bar.com",
            "select",
            ".exit",
        ]
    end
end
