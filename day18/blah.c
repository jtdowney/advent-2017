#include <stdio.h>  // BUFSIZ, fgets, printf
#include <stdlib.h> // atoll
#include <ctype.h>  // isalpha
#include <string.h> // strdup, strcmp

#define SEP " \n\t"
#define MAXPROGLEN 100
#define MAXQ 10000

typedef long long int LLI;

struct {
        char *code; // opcode
        char *arg1; // first operand
        char *arg2; // second operand
} core[MAXPROGLEN];

struct task {
        LLI regs[0xff]; // indexed by 8-bit ascii code (extravagant!)
        LLI next;       // offset of next instruction
        LLI sent;       // # of messages sent
        int head;       // head of message queue
        int tail;       // tail of message queue
        LLI msgs[MAXQ]; // message queue
} task[2];

int
qempty(int id)
{
        return task[id].head == task[id].tail;
}

void
enqueue(int id, LLI v)
{
        task[id].msgs[task[id].tail] = v;
        task[id].tail = (task[id].tail + 1) % MAXQ;
        ++task[1 - id].sent;
}

LLI
dequeue(int id)
{
        LLI v = task[id].msgs[task[id].head];
        task[id].head = (task[id].head + 1) % MAXQ;
        return v;
}

char *
dupstr(char *s)
{
        if (s == NULL)
                return s;
        return strdup(s);
}

int
load(void)
{
        char buf[BUFSIZ];
        int next;

        next = 0;
        while (fgets(buf, sizeof buf, stdin) != NULL) {
                core[next].code = dupstr(strtok(buf, SEP));
                core[next].arg1 = dupstr(strtok(NULL, SEP));
                core[next].arg2 = dupstr(strtok(NULL, SEP));
                ++next;
        }
        return next;
}

// extract a value from the 2nd operand, if any
LLI
value(struct task *t, char *arg)
{
        if (arg == NULL)
                return 0; // anything, return value won't be used
        if (isalpha(*arg))
                return t->regs[*arg];
        return atoll(arg);
}

// return 0 if not running, 1 if blocked
int
run(int id, int n)
{
        for (struct task *t = &task[id]; t->next >= 0 && t->next < n; ++t->next) {
                LLI arg1 = value(t, core[t->next].arg1);
                LLI arg2 = value(t, core[t->next].arg2);

                if (strcmp(core[t->next].code, "snd") == 0) {
                        enqueue(1 - id, arg1);
                } else if (strcmp(core[t->next].code, "set") == 0) {
                        t->regs[*core[t->next].arg1] = arg2;
                } else if (strcmp(core[t->next].code, "add") == 0) {
                        t->regs[*core[t->next].arg1] += arg2;
                } else if (strcmp(core[t->next].code, "mul") == 0) {
                        t->regs[*core[t->next].arg1] *= arg2;
                } else if (strcmp(core[t->next].code, "mod") == 0) {
                        t->regs[*core[t->next].arg1] %= arg2;
                } else if (strcmp(core[t->next].code, "rcv") == 0) {
                        if (qempty(id))
                                return 1;
                        t->regs[*core[t->next].arg1] = dequeue(id);

                } else { // jgz
                        if (arg1 > 0)
                                t->next += arg2 - 1;
                }
        }
        return 0;
}

int
main(void)
{
        task[0].regs['p'] = 0;
        task[1].regs['p'] = 1;

        int n = load();
        int running[2] = {1, 1};

        for (;;) {
                running[0] = run(0, n);     // run until blocked or ended
                running[1] = run(1, n);     // run until blocked or ended
                if (!running[0] && !running[1])
                        break;
                if (running[0] && qempty(0) && running[1] && qempty(1))
                        break; // deadlocked
        }
        printf("%lld\n",task[1].sent);
        return 0;
}
