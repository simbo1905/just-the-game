/**
 * Just The Game Engine
 * Core logic for flash-card quiz game
 */

class GameEngine {
    constructor() {
        this.questions = [];
        this.currentQuestionIndex = 0;
        this.score = 0;
        this.answered = 0;
        this.gameState = 'loading'; // loading, ready, playing, question-answered, finished
        this.currentQuestion = null;
        this.userAnswer = null;
        this.gameData = null;
    }

    /**
     * Load questions from JSON data
     * @param {Object} gameData - The complete game data object
     */
    async loadQuestions(gameData) {
        try {
            this.gameData = gameData;
            this.questions = gameData.questions || [];
            this.shuffleQuestions();
            this.currentQuestionIndex = 0;
            this.score = 0;
            this.answered = 0;
            this.gameState = 'ready';
            this.currentQuestion = null;
            this.userAnswer = null;
            
            console.log(`Loaded ${this.questions.length} questions`);
            return true;
        } catch (error) {
            console.error('Error loading questions:', error);
            this.gameState = 'error';
            return false;
        }
    }

    /**
     * Shuffle questions for random order
     */
    shuffleQuestions() {
        for (let i = this.questions.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [this.questions[i], this.questions[j]] = [this.questions[j], this.questions[i]];
        }
    }

    /**
     * Start the game
     */
    startGame() {
        if (this.gameState !== 'ready') {
            throw new Error('Game not ready to start');
        }
        
        this.currentQuestionIndex = 0;
        this.score = 0;
        this.answered = 0;
        this.gameState = 'playing';
        this.loadCurrentQuestion();
    }

    /**
     * Load the current question
     */
    loadCurrentQuestion() {
        if (this.currentQuestionIndex >= this.questions.length) {
            this.gameState = 'finished';
            return null;
        }

        this.currentQuestion = this.questions[this.currentQuestionIndex];
        this.userAnswer = null;
        this.gameState = 'playing';
        console.log(`🎯 DEBUG: Question ${this.currentQuestionIndex + 1} correct answer is index ${this.currentQuestion.correctAnswer}`);
        return this.currentQuestion;
    }

    /**
     * Submit an answer for the current question
     * @param {number} answerIndex - Index of the selected answer
     */
    submitAnswer(answerIndex) {
        if (this.gameState !== 'playing' || !this.currentQuestion) {
            throw new Error('No active question to answer');
        }

        if (answerIndex < 0 || answerIndex >= this.currentQuestion.choices.length) {
            throw new Error('Invalid answer index');
        }

        this.userAnswer = answerIndex;
        const isCorrect = answerIndex === this.currentQuestion.correctAnswer;
        const mode = this.getMetadata().mode || 'hard';

        // In easy mode, if the answer is incorrect, we don't advance the game state.
        // We just provide feedback that the answer was wrong.
        if (mode === 'easy' && !isCorrect) {
            return {
                correct: false,
                correctAnswerIndex: this.currentQuestion.correctAnswer,
                userAnswerText: this.currentQuestion.choices[answerIndex],
                explanation: 'That wasn\'t correct. Try another answer!',
                mode: 'easy'
            };
        }
        
        this.answered++;
        
        if (isCorrect) {
            this.score++;
        }

        this.gameState = 'question-answered';
        
        return {
            correct: isCorrect,
            correctAnswerIndex: this.currentQuestion.correctAnswer,
            correctAnswerText: this.currentQuestion.choices[this.currentQuestion.correctAnswer],
            userAnswerText: this.currentQuestion.choices[answerIndex],
            explanation: this.currentQuestion.explanation,
            mode: mode
        };
    }

    /**
     * Move to the next question
     */
    nextQuestion() {
        if (this.gameState !== 'question-answered') {
            throw new Error('Must answer current question first');
        }

        this.currentQuestionIndex++;
        return this.loadCurrentQuestion();
    }

    /**
     * Get current game statistics
     */
    getStats() {
        return {
            score: this.score,
            answered: this.answered,
            total: this.questions.length,
            percentage: this.answered > 0 ? Math.round((this.score / this.answered) * 100) : 0,
            remaining: this.questions.length - this.currentQuestionIndex,
            gameState: this.gameState
        };
    }

    /**
     * Get current question data
     */
    getCurrentQuestion() {
        return this.currentQuestion;
    }

    /**
     * Reset the game to initial state
     */
    reset() {
        this.currentQuestionIndex = 0;
        this.score = 0;
        this.answered = 0;
        this.gameState = this.questions.length > 0 ? 'ready' : 'loading';
        this.currentQuestion = null;
        this.userAnswer = null;
    }

    /**
     * Get game metadata
     */
    getMetadata() {
        return this.gameData?.metadata || {};
    }

    /**
     * Check if game is finished
     */
    isFinished() {
        return this.gameState === 'finished';
    }

    /**
     * Check if question has been answered
     */
    isQuestionAnswered() {
        return this.gameState === 'question-answered';
    }

    /**
     * Check if game is ready to play
     */
    isReady() {
        return this.gameState === 'ready';
    }

    /**
     * Check if currently playing
     */
    isPlaying() {
        return this.gameState === 'playing';
    }
}