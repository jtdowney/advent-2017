def run(input, id, tx, rx)
  regs = Hash.new(0)
  regs[:p] = id
  vals_received = 0
  pc = -1
  resolve = ->(y) { y.is_a?(Integer) ? y : regs[y] }

  while (pc += 1) >= 0 && (inst = input[pc])
    case inst[0]
    when :snd
      tx << resolve[inst[1]]
    when :set
      regs[inst[1]] = resolve[inst[2]]
    when :add
      regs[inst[1]] += resolve[inst[2]]
    when :mul
      regs[inst[1]] *= resolve[inst[2]]
    when :mod
      regs[inst[1]] %= resolve[inst[2]]
    when :rcv
      if tx.object_id == rx.object_id
        # Part 1!
        return rx[-1] if resolve[inst[1]] != 0
      else
        val = nil
        # Oh noes, a spinlock.
        val = rx[vals_received] until val
        vals_received += 1
        regs[inst[1]] = val
      end
    when :jgz
      pc += (resolve[inst[2]] - 1) if resolve[inst[1]] > 0
    else raise "Unknown instruction #{inst}"
    end
  end
end

insts = (ARGV.empty? ? DATA : ARGF).each_line.map { |l|
  inst, *args = l.split
  [inst.to_sym, *args.map { |a| a.match?(/-?\d+/) ? a.to_i : a.to_sym }].freeze
}.freeze

sound = []
puts run(insts, 0, sound, sound)

send0 = []
send1 = []

t0 = Thread.new { run(insts, 0, send0, send1) }
t1 = Thread.new { run(insts, 1, send1, send0) }
t2 = Thread.new {
  loop {
    puts send1.size
    sleep 1
  }
}
