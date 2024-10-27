# Author

### 2.0.0
Separates /prompts to distinguished table
- /translates/
- /checkbots/
- /tts/
- /stt/

### < 2.0.0
- login
- register
- change-password
- reset-password
- Add response format to audio speech endpoint
- /v1/prompts/
- /v1/students/
- /v1/subscriptions/
- /v1/users/

### < 1.4.0
- Add phonetic transcriptions promptype (1.3.5)
- Change model from gpt-3.5-turbo to gpt-4o-mini (1.3.5)
- Add Transcription endpoint (1.3.4)
- Add created_at to users table (1.3.3)
- Add updated_at to table which doesn't have updated_at (1.3.3)
- Split Prompt and Text to speech endpoint (1.3.2)
- Remove sentry and tracing (1.3.1)
- Update diesel to 2.2.1 (1.3.0)
- Update sentry to 0.34.0 (1.3.0)
- Add allow dead code (1.3.0)
- Add tracing instrument (1.3.0)

### < 1.3.0

- Update sentry (1.2.9)
- Remove image to text api (1.2.8)
- Update PromptType GrammarCheck to Checkbot (1.2.7)
- Remove payasyougo Payment plan (1.2.6)
- Update free prompt limit to 10 (1.2.5)
- Update gpt-3.5-turbo-16k to gpt-3.5-turbo due to depreciation notice (1.2.4)