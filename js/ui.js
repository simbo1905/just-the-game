/**
 * UI Controller for Just The Game
 * Handles all user interface interactions and updates
 */

class UIController {
    constructor(gameEngine) {
        this.game = gameEngine;
        this.elements = {};
        this.init();
    }

    /**
     * Initialize UI elements and event listeners
     */
    init() {
        // Cache DOM elements
        this.elements = {
            gameContainer: document.getElementById('game-container'),
            loadingScreen: document.getElementById('loading-screen'),
            startScreen: document.getElementById('start-screen'),
            gameScreen: document.getElementById('game-screen'),
            resultScreen: document.getElementById('result-screen'),
            finishScreen: document.getElementById('finish-screen'),
            gameTitle: document.getElementById('game-title'),
            gameDescription: document.getElementById('game-description'),
            questionCounter: document.getElementById('question-counter'),
            scoreDisplay: document.getElementById('score-display'),
            questionText: document.getElementById('question-text'),
            choicesContainer: document.getElementById('choices-container'),
            resultIcon: document.getElementById('result-icon'),
            resultText: document.getElementById('result-text'),
            explanationText: document.getElementById('explanation-text'),
            nextButton: document.getElementById('next-button'),
            finalScore: document.getElementById('final-score'),
            finalPercentage: document.getElementById('final-percentage'),
            playAgainButton: document.getElementById('play-again-button'),
            questionSetTilesContainer: document.getElementById('question-set-tiles')
        };

        this.bindEvents();
    }

    /**
     * Bind event listeners
     */
    bindEvents() {
        if (this.elements.nextButton) {
            this.elements.nextButton.addEventListener('click', () => this.nextQuestion());
        }
        
        if (this.elements.playAgainButton) {
            this.elements.playAgainButton.addEventListener('click', () => this.restartGame());
        }
    }

    /**
     * Load game data and initialize
     */
    async loadGame() {
        this.showScreen('loading');
        
        try {
            const response = await fetch('./data/questions.json');
            const gameData = await response.json();
            
            const success = await this.game.loadQuestions(gameData);
            if (success) {
                this.updateStartScreen();
                this.showScreen('start');
            } else {
                this.showError('Failed to load game data');
            }
        } catch (error) {
            console.error('Error loading game:', error);
            this.showError('Failed to load game. Please check your connection.');
        }
    }

    /**
     * Load embedded game data (for packed version)
     */
    async loadEmbeddedGame() {
        this.showScreen('loading');
        
        try {
            // Use embedded data instead of fetching
            if (window.EMBEDDED_GAME_DATA) {
                const success = await this.game.loadQuestions(window.EMBEDDED_GAME_DATA);
                if (success) {
                    this.updateStartScreen();
                    this.showScreen('start');
                } else {
                    this.showError('Failed to load embedded game data');
                }
            } else {
                this.showError('No embedded game data found');
            }
        } catch (error) {
            console.error('Error loading embedded game:', error);
            this.showError('Failed to load embedded game data.');
        }
    }

    /**
     * Load embedded question sets (for packed version with multiple sets)
     */
    async loadEmbeddedQuestionSets() {
        this.showScreen('loading');
        
        try {
            if (window.EMBEDDED_QUESTION_SETS) {
                this.questionSets = window.EMBEDDED_QUESTION_SETS;
                
                // Load the first available question set by default
                const firstKey = Object.keys(this.questionSets)[0];
                if (firstKey) {
                    const success = await this.game.loadQuestions(this.questionSets[firstKey]);
                    if (success) {
                        this.currentQuestionSetKey = firstKey;
                        // Set the selector to the first key
                        if (this.elements.questionSetSelector) {
                            this.elements.questionSetSelector.value = firstKey;
                        }
                        this.updateStartScreenWithQuestionSets();
                        this.showScreen('start');
                    } else {
                        this.showError('Failed to load question set');
                    }
                } else {
                    this.showError('No question sets found');
                }
            } else {
                // Fallback to single embedded data for backwards compatibility
                this.loadEmbeddedGame();
            }
        } catch (error) {
            console.error('Error loading embedded question sets:', error);
            this.showError('Failed to load question sets.');
        }
    }

    /**
     * Handle question set selection change
     */
    async onQuestionSetChange(selectedKey) {
        if (this.questionSets && this.questionSets[selectedKey]) {
            const success = await this.game.loadQuestions(this.questionSets[selectedKey]);
            if (success) {
                this.currentQuestionSetKey = selectedKey;
                this.updateQuestionSetInfo();
            }
        }
    }

    /**
     * Update start screen with multiple question sets as tiles
     */
    updateStartScreenWithQuestionSets() {
        if (this.questionSets && this.elements.questionSetTilesContainer) {
            this.elements.questionSetTilesContainer.innerHTML = ''; // Clear existing tiles

            for (const key in this.questionSets) {
                if (Object.hasOwnProperty.call(this.questionSets, key)) {
                    const questionSet = this.questionSets[key];
                    const metadata = questionSet.metadata || {};

                    const tile = document.createElement('div');
                    tile.className = 'question-set-tile';
                    tile.dataset.key = key;

                    const title = document.createElement('h3');
                    title.className = 'tile-title';
                    title.textContent = metadata.title || key.replace(/_/g, ' ');

                    const description = document.createElement('p');
                    description.className = 'tile-description';
                    description.textContent = metadata.description || 'A set of questions on this topic.';

                    tile.appendChild(title);
                    tile.appendChild(description);

                    // Add hover events to show description below tiles
                    tile.addEventListener('mouseenter', () => {
                        if (this.elements.gameDescription) {
                            this.elements.gameDescription.textContent = metadata.description || 'A set of questions on this topic.';
                        }
                    });
                    
                    tile.addEventListener('mouseleave', () => {
                        if (this.elements.gameDescription) {
                            this.elements.gameDescription.textContent = 'Select a topic to begin.';
                        }
                    });

                    tile.addEventListener('click', () => this.startGame(key));

                    this.elements.questionSetTilesContainer.appendChild(tile);
                }
            }
        }
    }

    /**
     * Update start screen with game metadata
     */
    updateStartScreen() {
        // This function is now simplified as tiles handle most of the info.
        // We can set a generic title or description if needed.
        if (this.elements.gameTitle) {
            this.elements.gameTitle.textContent = "Just The Game";
        }
        if (this.elements.gameDescription) {
            this.elements.gameDescription.textContent = "Select a topic to begin your learning journey.";
        }
    }

    /**
     * Start the game with a specific question set
     * @param {string} questionSetKey - The key for the selected question set
     */
    async startGame(questionSetKey) {
        if (!questionSetKey || !this.questionSets[questionSetKey]) {
            this.showError('Please select a valid question set to start.');
            return;
        }
        
        this.currentQuestionSetKey = questionSetKey;
        const selectedSet = this.questionSets[this.currentQuestionSetKey];

        await this.game.loadQuestions(selectedSet);
        this.game.startGame();
        this.showQuestion();
        this.showScreen('game');
    }

    /**
     * Display current question
     */
    showQuestion() {
        const question = this.game.getCurrentQuestion();
        const stats = this.game.getStats();
        
        if (!question) {
            this.showFinishScreen();
            return;
        }

        // Update question counter and score
        if (this.elements.questionCounter) {
            this.elements.questionCounter.textContent = 
                `Question ${stats.answered + 1} of ${stats.total}`;
        }
        
        if (this.elements.scoreDisplay) {
            this.elements.scoreDisplay.textContent = 
                `Score: ${stats.score}/${stats.answered}`;
        }

        // Display question
        if (this.elements.questionText) {
            this.elements.questionText.textContent = question.question;
        }

        // Create choice buttons
        this.createChoiceButtons(question.choices);
    }

    /**
     * Create clickable choice buttons
     */
    createChoiceButtons(choices) {
        if (!this.elements.choicesContainer) return;

        this.elements.choicesContainer.innerHTML = '';
        
        choices.forEach((choice, index) => {
            const button = document.createElement('button');
            button.className = 'choice-button';
            button.textContent = choice;
            button.onclick = () => {
                console.log(`Choice ${index} clicked: ${choice}`);
                this.selectAnswer(index);
            };
            
            this.elements.choicesContainer.appendChild(button);
        });
        console.log(`Created ${choices.length} choice buttons`);
    }

    /**
     * Handle answer selection
     */
    selectAnswer(answerIndex) {
        console.log(`selectAnswer called with index: ${answerIndex}`);
        
        const result = this.game.submitAnswer(answerIndex);
        console.log('Answer submitted, result:', result);

        const buttons = this.elements.choicesContainer.querySelectorAll('.choice-button');
        const selectedButton = buttons[answerIndex];

        // Easy mode, incorrect answer: just mark it red and stop.
        if (result.mode === 'easy' && !result.correct) {
            selectedButton.classList.add('incorrect');
            selectedButton.disabled = true; // Prevent clicking the same wrong answer
            console.log('Easy mode, incorrect answer. Allowing another try.');
            return; // Stop here, don't show result screen
        }

        // Hard mode OR Correct answer in easy mode: proceed to result screen.
        
        // Disable all choice buttons
        buttons.forEach(button => button.disabled = true);
        
        // Highlight correct and incorrect answers
        buttons.forEach((button, index) => {
            if (index === result.correctAnswerIndex) {
                button.classList.add('correct');
            } else if (index === answerIndex && !result.correct) {
                // This case only happens in hard mode now
                button.classList.add('incorrect');
            }
        });

        // Show result after a brief delay
        setTimeout(() => {
            console.log('About to show result screen...');
            this.showResult(result);
        }, 800);
    }

    /**
     * Show answer result and explanation
     */
    showResult(result) {
        if (this.elements.resultIcon) {
            this.elements.resultIcon.textContent = result.correct ? '✅' : '❌';
        }
        
        if (this.elements.resultText) {
            this.elements.resultText.textContent = result.correct 
                ? 'Correct!' 
                : `Incorrect. The correct answer was: ${result.correctAnswerText}`;
        }
        
        if (this.elements.explanationText) {
            this.elements.explanationText.textContent = result.explanation;
        }

        console.log('Showing result screen...');
        this.showScreen('result');
    }

    /**
     * Move to next question
     */
    nextQuestion() {
        const nextQ = this.game.nextQuestion();
        
        if (nextQ) {
            this.showQuestion();
            this.showScreen('game');
        } else {
            this.showFinishScreen();
        }
    }

    /**
     * Show final results
     */
    showFinishScreen() {
        const stats = this.game.getStats();
        
        if (this.elements.finalScore) {
            this.elements.finalScore.textContent = `${stats.score} out of ${stats.total}`;
        }
        
        if (this.elements.finalPercentage) {
            this.elements.finalPercentage.textContent = `${stats.percentage}%`;
        }

        this.showScreen('finish');
    }

    /**
     * Restart the game by reloading the page.
     */
    restartGame() {
        console.log('🔄 [UI] Restarting game by reloading the page.');
        location.reload();
    }

    /**
     * Show specific screen
     */
    showScreen(screenName) {
        // Hide all screens
        Object.values(this.elements).forEach(element => {
            if (element && element.classList && element.classList.contains('screen')) {
                element.style.display = 'none';
            }
        });

        // Show requested screen
        const screenMap = {
            'loading': this.elements.loadingScreen,
            'start': this.elements.startScreen,
            'game': this.elements.gameScreen,
            'result': this.elements.resultScreen,
            'finish': this.elements.finishScreen
        };

        const targetScreen = screenMap[screenName];
        if (targetScreen) {
            targetScreen.style.display = 'block';
        }
    }

    /**
     * Show error message
     */
    showError(message) {
        // Simple error display - could be enhanced with a proper error screen
        if (this.elements.loadingScreen) {
            this.elements.loadingScreen.innerHTML = `
                <div class="error">
                    <h2>Error</h2>
                    <p>${message}</p>
                    <button onclick="location.reload()">Retry</button>
                </div>
            `;
        }
    }
}