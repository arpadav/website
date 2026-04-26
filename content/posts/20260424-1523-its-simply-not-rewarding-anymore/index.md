# its simply not rewarding anymore

## on software

for someone who has been actively programming for more than a decade, the title says it all.

i have been seeing similar sentiment floating around online. people like [andrej karpathy](https://x.com/karpathy/status/2015883857489522876) summarized it best with ones workflow moving from 80% manual + autocomplete coding with 20% agentic, to 20% manual + autocomplete with 80% agentic. this is true for myself as well. working for a non-profit predominately for the US government, it was still difficult to even get auto-completion capabilities since the work involved controlled unclassified information (CUI) which had to be controlled, and the US sponsors did not want big-tech to train on critical software running their infrastructure, wartime capabilities, security, and more.

however, since leaving that organization i have dipped my toes into agentic coding on some [personal projects](https://arpadvoros.com/projects/#rust-crates) as well my startup [Inspectra](https://inspectra.us) and well... it is simply too stupid NOT to use it. it would be a lie to say that this capability significantly speeds up development time, and i would feel like a complete luddite returning to manually programming, every character, every keystroke. i have no idea how the US government has since transformed, since i know there were talks about hosting big-tech's LLM's on local infra, but AFAIK private industry was adamantly against giving up on their secret sauce.

### paradigm shift

if youre in my close circle, you know ive been harping on about this: the lows and highs are affected the most. what do i mean by this? i frequently see videos online about being a 10x developer being laid off, or a junior dev who has been 9 months out of the job market. and this honestly isnt the case for just software, its the case for most industries

the _low_ are the juniors who are getting out of school, trying their chance at an intro job. its obvious **why** they are struggling - they can simply be replaced by ai. why hire someone and pay them 60-80k USD when you can have a developer kick off AI agents, either locally, or use a subscription, for 3k per annum?

the _high_ are the professionals who have been in the industry for decades. they have aqcuired vast amounts of knowledge, know how to navigate a professional landscape without over-engineering or under-abstracting, but the golden mean on development. so **why** do i believe they are at higher risk than the mid tier? same reason as the _low_: pay. a senior developer can be getting paid anywhere from 300-500k USD, and its easier to have a mid-tier engineer with the "knowledge" of an LLM = output of a senior dev. the law is probably logarithmic in some way, where the gains AI gives for mid-tier engineers is 100% whereas for a senior developer it might be 20-30%.

as a result, it makes more sense to keep the mid level workforce "supercharged" by this eldritch horror of a technology, lower pay, higher thruput, where the mid-level engineers are expected to do the work for all 3 _low_ _mid_ and _high_.

this is obviously a huge oversimplification

### industry mindset

people who also know me know my criticism i have for those who wear this "software developer hat" but dont ***truly care*** about the compute, the science, the gross inefficiency we see being taken place.

people who harness the mindset "why should i know leetcode when AI can do it for me?" or "why should i learn this specific abstraction when AI can do it for me?" is probably the worst thing an individual can say. there is a clear difference between knowledge and intelligence, where you are the intelligent one and AI is the knowledgable one, and its a waltz that balances one another out. however, if you actively choose to not educate yourself and _introduce_ a type of knowledge to the forefront of this LLM's "mind" then you are not the type of developer nor human being i would ever want to be friends with

its like actively choosing not to learn the simplest and most powerful parts of a language and letting an AI do some weird work-around solution for it. for example:

#### rust trait extension

the amount of times i had to explain an AI about this, even when it had context from a [general rust coding agent](https://github.com/VoltAgent/awesome-claude-code-subagents/blob/main/categories/02-language-specialists/rust-engineer.md)

```rust
trait Walk {
    fn walk(&self) -> u8;
}

trait Run {
    fn run(&self) -> u16;
}

impl<T> Run for T
where
    T: Walk
{
    fn run(&self) -> u16 {
        (self.walk() as u16) << 1
    }
}
```

#### rust enum dispatch

or this

```rust
enum Client {
    AbcClient(AbcClient),
    XyzClient(XyzClient),
}

trait IsClient {
    fn name(&self) -> &'static str;
}

// impl `IsClient` for each sub-client, then

impl IsClient for Client {
    fn name(&self) -> &'static str {
        match self { .. } // <-- call each here
    }
}
```

which is the single most idiomatic way to use the power of rust traits, enumerations, and more. (off topic, but more often than not i have a copy-paste macro which does these impls automatically, but they tend to be project specific) and i wouldnt have _known_ this after painstakingly writing in rust the past 5 years and how i always converged back to these simple powerful patterns, and these developers who let AI agents roam free without learning fundamentals and the powerful language capabilities, tooling, and more, then the world will get ever more ***lazy*** and ***slow*** and other people, or even more AI, have to come in and attempt to fix the deliberate mess that **you** caused.

## on ai

### progress, and best COA

best course-of-action is: we need to ***slow down***. i remember seeing [this article make the rounds a couple months back](https://hbr.org/2026/02/ai-doesnt-reduce-work-it-intensifies-it) that describes how work has been increasingly intensified, since one human + AI can output more than one human without it. as a result, more humans can be let go, one human + AI now has to do the work of 2-3 humans. people are out of jobs, people are left overworked.

the big-picture can still be promising. we have to slow down and assess the end-goal. we want progress, but what are we progressing toward? a better life? if so, then we want to live without work, and have it all automated. then does our current economic structure support this? if not, do we have to change it? do we have to add novel capabilities? im not advocating for one thing or another. people talk about universal basic income as if it can manifest itself out of thin air, but if we are progressing to make our lives easier and have to do less work, then those working shouldnt be over worked and those not working shouldnt be left to struggle.

### automation

industrial revolution came, people lost their jobs and craftsmanship due to automation, people worked in factories.

there are stark similarities but also contrasts between that boom and this ongoing one. for one, both are resource and monetarily limited. for two, people have to lose a bit of humanity to use the machine for the sake of progress. 4 3, people are fatigued and working more than ever - anyone i talk to in any industry can attribute this. 

however, one difference i cant shake is the fact that the industrial revolution produced goods, but this current revolution is producing services. the american, and moreover, the western economy post ww2 saw a significant down-turn in their manufacturing as we strictly have become a service economy. other countries that manufacturing was offloaded to still had poor working conditions, and [some were able to lift themselves out to become a more well-rounded economy](https://en.wikipedia.org/wiki/Reform_and_opening_up#Economic_performance). so if our economy makes up majority services, which this current boom is automating, then whats left?

### taxes

back to UBI. the solution is more complicated than a nobody like myself can comprehend, but the simple idea of:

#### before

1. workers got paid, had income tax, social security, and medicare
2. income tax funds government
3. social security funds retirement
4. medicare funds aid

and note that income tax, social security, and medicare make up for a combined 85% of the US governments revenue in 2026: [https://fiscaldata.treasury.gov/americas-finance-guide/government-revenue/#sources-of-federal-revenue](https://fiscaldata.treasury.gov/americas-finance-guide/government-revenue/#sources-of-federal-revenue), with income tax being 50%.

#### current

1. less workers, lets say, 20% lose their job
2. federal funding cut by ***17%***

not only does the government highly likely NOT want this, but more people will be applying to unemployement, which affects them even more.

#### proposed

the solution is to make up the amount lost by introducing an "income tax" for those workers lost, from the corporations who are using the AI. this amount lost can help re-introduce some of the governments budget, but also help for a UBI fund.

i know many people like to harp on AI companies and how THEY should pay for their datacenters and they are causing all these issues with power and whatnot, but there is a clear demand for it, and they are almost all running in the negative to keep up with demand. it wouldnt make sense to introduce further tax onto them, at this moment in time.

## closing

money is money is money is money, and at the end of the day humans want to be human and live free without worry. that is what we should be progressing towards. this shift, however it plays out, should help in the complete automation of menial tasks while giving humans their freedom back. but at the current rate, its just causing more suffering and uncertainty, and we need to significantly take a step back and look at ourselves. similar to how there was a life before the internet, thats never going away. just like how now there is a life without consumer-language intellgence of the artifical variety, this is ***never*** going away. 

