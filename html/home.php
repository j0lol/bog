head

<div class="wrapper">
    todo navbar

    <script>
      Array.from(document.querySelectorAll('select'), (input) => {
        let parent = input.parentNode;

        function updateSize() {
          parent.dataset.selectAutoExpand = input.value
        }

        input.addEventListener('input', updateSize);

        updateSize();
      });
    </script>
    <main>
        <?php
        $pronouns = ["she/her", "they/them", "it/its"];
        shuffle($pronouns);

        $genders = [
            "𐂂",
            "\u{2400}",
            "\u{2205}",
            "girl?",
            "stolen",
            "not",
            // $_SERVER["REMOTE_ADDR"],
        ];
        shuffle($genders);

        function makeSelect(array $list, bool $funky = false)
        {
            echo "<select>";
            foreach ($list as $item) {
                $punctuation = "";

                if ($funky) {
                    $punctuation = [".", "!"][rand(0, 1)];
                }

                echo "<option>" . $item . $punctuation . "</option>";
            }
            echo "</select>";
        }
        ?>

        <h1 class="fancy page-head">Hi!</h1>

        <?php speech_start(SpeechCharacter::Deer, SpeechEmotion::Neutral); ?>
        <p>I'm Jo.
            <label id="my-pronouns">My pronouns are <?php makeSelect(
                $pronouns,
            ); ?></label>
            and
            <label>my gender is <?php makeSelect($genders, false); ?></label>
        </p>

        <p>I'm a CompSci graduate from the University of Sussex. <small><a href="/contact">(Hire me!)</a></small></p>

        <?php speech_end(); ?>

        <details style="margin-top: 1rem;">
            <summary>What's your pronouns? </summary>

            <label id="pronouns-blurb"> <input id="pronouns-choice"> </label>
            <button id="pronouns-submit" type="button" onclick="steal_pronouns()">Submit</button>

            <script>
                function steal_pronouns() {
                    let input_field = document.querySelector("#pronouns-choice");
                    let pronouns = input_field.value;
                    input_field.value = "";

                    document.querySelector("#pronouns-blurb").innerHTML = "haha! mine now!";
                    document.querySelector("#my-pronouns").innerHTML = "My pronouns are <b>" + pronouns + "</b>";
                    document.querySelector("#pronouns-submit").remove();
                }
            </script>
        </details>


        <h2>What do you do?</h2>
        <p>I mainly write software, and study in the art of writing software. My specialities lie in writing correct, robust code. I love to read and write documentation, and to double-check my work. Outside of development, I like to draw and write, and cook meals with a good splash of umami. I am conversational in a constructed language, toki pona. </p>
        <p>Here's what I'm interested in right now!</p>
        <ul style="margin-top: -0.5rem;">
        <?php
        $interests = [
            "Rust <small>(self explanatory)</small>",
            "Making small stuff with PHP",
            "WebGPU for fast, write-once-run-anywhere rendering",
            "Small websites, small communities",
            "Idiomatic vanilla Javascript, modern CSS",
            "Clean design",
        ];
        foreach ($interests as $interest) {
            echo "<li>" . $interest . "</li>";
        }
        ?>
        </ul>

        <p style="margin-bottom: 0.25em">
            Here's what I'm doing on the computer:
        </p>
        <div>
            <blockquote class="small">Slowly hacking away on <a href="https://github.com/j0lol/vee">Vee Face Library</a>, a pure Rust Mii renderer and parser.</blockquote>
        </div>

        <p style="margin-bottom: 0.25em">
            And here's what I'm doing offline:
        </p>
        <div>
            <blockquote class="small"">Not much! I'm still kind of recovering from burnout. I've been playing a bit of chess though. <a href="https://lichess.org/@/j0lol">Not very good at it yet.</a></blockquote>
        </div>

        <h2>More info</h2>
        <ul>
            <li>
                If you want to see what I've done before, see <a href="<?php echo $router->generate(
                    "projects",
                ); ?>">my
                    projects</a>!
            </li>

            <li>
                If you want to see how I did my projects, see <a href="<?php echo $router->generate(
                    "blog-index",
                ); ?>">my
                    blog</a>!
            </li>
        </ul>

        <p>
            I'm always working on something! If you're curious,
            <a href="<?= $router->generate(
                "contact",
            ) ?>">contact me</a>, or check out my <a href="https://github.com/j0lol">git repos</a>!
        </p>
    </main>

    <div style="background: var(--bg-surface1); padding: 0rem 1rem;" class="tablet-show">
        <hr class="frontmatter pilcrow">

    </div>

    <div class="page">

        <?php speech_start(SpeechCharacter::Deer, SpeechEmotion::Happy); ?>
        <p>
            Thanks for visiting my website! Here's some friend's sites. These are the 88&times;31 buttons that my friends made to link to their websites. Click them to go to them!
        </p>
        <?php speech_end(); ?>

        <br>
        <details>
            <summary>Embed code for my website</summary>

            <pre><code class="language-html">&#x3C;a rel=&#x22;noreferrer&#x22; href=&#x22;https://j0.lol&#x22;&#x3E;
    &#x3C;img src=&#x22;https://j0.lol/static/badges/j0.gif&#x22; alt=&#x22;Logo: j0, with subtitle &#x27;deer thing&#x27;. To the side, there is a purple deer with yellow features. Various elements flicker.&#x22;&#x3E;
&#x3C;/a&#x3E;</code></pre>

            <p>Note: I run (self-hosted) analytics on my site! If you put me on your website, someone will inevitably click it without referrer blocking, meaning that I will know! And I will probably add you here if you do that!</p>

        </details>
        <br>
        <div class="_88x31s">
            <?php
            $buttons = [
                "meihapps.gif" => [
                    "Logo: mei happs. Monospaced text. Pink border.",
                    "https://meihapps.gay",
                ],
                "finn.gif" => [
                    "Logo: cr0wbar. Handwritten text. The 0 has what appears to be a shiny gem inside of it.",
                    "https://cr0wbar.dev",
                ],
                "cadence_now.png" => [
                    "Logo: cadence Now!. The first part of the text is in an 'outrun' vaporwave-type style. The final part is handwritten. There is what appears to be a purple eye with a yellow iris to the left of the text. Enclosing the eye is a white letter c. There is a star banner on the bottom right.",
                    "https://cadence.moe",
                ],
                "barrow.png" => [
                    "Green text says 'Join Barr0wnet'. Enclosing the text is a scientific diagram of the chemical alpha-methyl.",
                    "http://alphamethyl.barr0w.net",
                ],
                "swiftys.gif" => [
                    "Swifty's HQ!",
                    "https://swiftyshq.neocities.org",
                ],
            ];

            foreach ($buttons as $file => [$alt, $link]) {
                printf(
                    '<a href="%s"><img class="raw" src="/static/badges/%s" alt="%s"> </a>',
                    $link,
                    $file,
                    $alt,
                );
            }
            ?>
        </div>
    </div>
</div>
