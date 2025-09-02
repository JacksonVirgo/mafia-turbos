# Mafia Turbos

Website to run quick games of mafia

## TODO:
Rough list (in some kind of order) of what I want to do:
1. Static layouts
2. Minimal auth via guest sessions (later Discord OAuth)
    - Simple `/whoami` to show session information
3. Sqlx/Postgres integrations (just a skeleton)
4. Single global chatroom
5. Presence and Reconnections
6. Lobbies
    - Create, Join, Leaving
7. Game "engine" skeleton for mountainous games
8. Voting integration for eliminations and night kills.
9. Private sub-lobbies for mafia chat, etc.
    - Allowing variants for phase-specific or "24/7" chats
10. Admin moderation controls
11. Simple roles and additional mechanics

After #11, the tool should be fairly adaptable and can be ported to more complex mafia games. I know a community that is needing a Town of Salem clone of sorts and if their dev team take too long and I beat them to this, they can just use this until they get their own bespoke version or just use mine I guess.

Note to myself to keep things modular so they can be extended to fit a variety of game types.

I saw HTMX has a websocket extension so I will try and use that unless it is too rigid and doesn't fit my needs, have not looked into that at all.
