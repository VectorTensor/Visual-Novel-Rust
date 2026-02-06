VAR met = false
VAR trust = 0

=== start ===
{ met == false:
    ~ met = true
    -> first_time
- else:
    -> repeat_time
}

=== first_time ===
# speaker:NPC
Hello. I don’t recognize you.

* "Just passing through."
    # speaker:NPC
    Then don’t cause trouble.
    -> DONE

* "I’m looking for work."
    ~ trust = trust + 1
    # speaker:NPC
    Maybe I’ll have something later.
    -> DONE

=== repeat_time ===
# speaker:NPC
Back again.

{ trust >= 2:
    # speaker:NPC
    You seem reliable.
    -> job_offer
- else:
    # speaker:NPC
    I’m still not convinced.
    -> smalltalk
}

=== smalltalk ===
* "Nice weather."
    # speaker:NPC
    It won’t last.
    -> DONE

* "I’ll come back later."
    # speaker:NPC
    Hmph.
    -> END

=== job_offer ===
# speaker:NPC
I have a task for you.

* "Tell me more."
    # speaker:NPC
    Meet me at dusk.
    -> END

* "Not interested."
    # speaker:NPC
    Then we’re done here.
    -> END
