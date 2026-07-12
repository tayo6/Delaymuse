#include "PluginEditor.h"
#include <cmath>

GradientKnob::GradientKnob(juce::AudioProcessorValueTreeState& s) : apvts(s)
{
    param = dynamic_cast<juce::AudioParameterFloat*>(apvts.getParameter("knob"));
    if (param) { target = param->get(); current = target; }
    setSize(460, 460);
    startTimerHz(60);
}

juce::Colour GradientKnob::lerpColor(float t)
{
    t = juce::jlimit(0.0f, 1.0f, t);
    auto green  = juce::Colour::fromRGB(34, 197, 94);
    auto yellow = juce::Colour::fromRGB(250, 204, 21);
    auto red    = juce::Colour::fromRGB(220, 38, 38);
    if (t <= 0.5f) return green.interpolatedWith(yellow, t * 2.0f);
    return yellow.interpolatedWith(red, (t - 0.5f) * 2.0f);
}

juce::Point<float> GradientKnob::angleToPos(juce::Point<float> c, float r, float a)
{
    return { c.x + std::cos(a) * r, c.y + std::sin(a) * r };
}

void GradientKnob::roundedLine(juce::Graphics& g, juce::Point<float> a, juce::Point<float> b, float w, juce::Colour col)
{
    g.setColour(col);
    g.drawLine(juce::Line<float>(a, b), w);
    auto rect = juce::Rectangle<float>(w, w);
    g.fillEllipse(rect.withCentre(a));
    g.fillEllipse(rect.withCentre(b));
}

void GradientKnob::timerCallback()
{
    constexpr float dt = 1.0f / 60.0f;
    const float l = 1.0f - std::exp(-dt * 18.0f);
    current += (target - current) * l;
    if (std::abs(current - target) < 0.005f) current = target;
    repaint();
}

void GradientKnob::updateFromMouse(juce::Point<float> p, bool force)
{
    auto c = getLocalBounds().toFloat().getCentre();
    float deg = std::atan2(p.y - c.y, p.x - c.x) * 180.0f / juce::MathConstants<float>::pi;
    if (deg < 0) deg += 360.0f;
    if (deg > DEAD_START && deg < DEAD_END) return;

    float rel = (deg >= START_DEG) ? (deg - START_DEG) : (deg + 360.0f - START_DEG);
    float nv = juce::jlimit(0.0f, 100.0f, rel / SWEEP_DEG * 100.0f);

    if (force || std::abs(nv - target) < 60.0f)
    {
        target = nv;
        if (param) param->setValueNotifyingHost(param->convertTo0to1(target));
    }
}

void GradientKnob::mouseDown(const juce::MouseEvent& e)
{
    if (param) param->beginChangeGesture();
    isDragging = true;
    updateFromMouse(e.position, true);
}
void GradientKnob::mouseDrag(const juce::MouseEvent& e) { updateFromMouse(e.position, false); }
void GradientKnob::mouseUp(const juce::MouseEvent&) { if (param) param->endChangeGesture(); isDragging = false; }
void GradientKnob::mouseWheelMove(const juce::MouseEvent&, const juce::MouseWheelDetails& w)
{
    target = juce::jlimit(0.0f, 100.0f, target + w.deltaY * 12.0f);
    if (param) param->setValueNotifyingHost(param->convertTo0to1(target));
}

void GradientKnob::paint(juce::Graphics& g)
{
    auto bounds = getLocalBounds().toFloat();
    auto center = bounds.getCentre();

    // x2 widths to match your request
    const float radius = 102.0f;
    const float sw = 32.0f; // was 16, now x2
    const float tickR = 142.0f;

    const float startRad = START_DEG * juce::MathConstants<float>::pi / 180.0f;
    const float sweepRad = SWEEP_DEG * juce::MathConstants<float>::pi / 180.0f;
    const float curT = juce::jlimit(0.0f, 1.0f, current / 100.0f);
    const float curRad = startRad + curT * sweepRad;
    const juce::Colour curCol = lerpColor(curT);

    // background like screenshot
    g.fillAll(juce::Colour::fromRGB(13, 13, 15));

    // inner maroon disc
    const float innerR = tickR - 8.0f;
    g.setColour(juce::Colour::fromRGB(58, 28, 28));
    g.fillEllipse(juce::Rectangle<float>(innerR * 2.0f, innerR * 2.0f).withCentre(center));

    // ticks - x2 width + dynamic size near pointer
    for (int i = 0; i <= 40; ++i)
    {
        float t = i / 40.0f;
        float rad = (START_DEG + t * SWEEP_DEG) * juce::MathConstants<float>::pi / 180.0f;
        bool major = (i % 10 == 0);

        float baseLen = major ? 18.0f : (i % 5 == 0 ? 13.0f : 9.0f);
        float baseW   = major ? 5.6f : 3.2f; // x2 from 2.8 / 1.6

        float dist = std::abs(t - curT);
        float prox = 1.0f - juce::jlimit(0.0f, 1.0f, dist / 0.12f);
        float scale = 1.0f + prox * 0.9f; // grows near knob

        float len = baseLen * scale;
        float w = baseW * scale;

        juce::Colour col = (t <= curT + 0.001f) ? lerpColor(t) : juce::Colour::fromRGB(85, 88, 95);
        auto p1 = angleToPos(center, tickR, rad);
        auto p2 = angleToPos(center, tickR + len, rad);
        roundedLine(g, p1, p2, w, col);
    }

    // grey track - same start as gradient, with rounded caps
    juce::Path grey;
    grey.startNewSubPath(angleToPos(center, radius, startRad));
    for (int i = 1; i <= 128; ++i)
    {
        float tt = i / 128.0f;
        grey.lineTo(angleToPos(center, radius, startRad + tt * sweepRad));
    }
    g.setColour(juce::Colour::fromRGB(62, 68, 77));
    g.strokePath(grey, juce::PathStrokeType(sw, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));

    // active gradient - starts exactly where grey starts, with rounded caps per segment
    if (curT > 0.001f)
    {
        const int segs = 180; // smooth
        for (int s = 0; s < segs; ++s)
        {
            float t0 = (float)s / segs * curT;
            float t1 = (float)(s + 1) / segs * curT;
            float a0 = startRad + t0 * sweepRad;
            float a1 = startRad + t1 * sweepRad;

            juce::Path seg;
            seg.startNewSubPath(angleToPos(center, radius, a0));
            seg.lineTo(angleToPos(center, radius, a1));

            g.setColour(lerpColor((t0 + t1) * 0.5f));
            g.strokePath(seg, juce::PathStrokeType(sw, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));
        }
    }

    // knob handle
    auto kp = angleToPos(center, radius, curRad);
    g.setColour(juce::Colour::fromFloatRGBA(0, 0, 0, 0.35f));
    g.fillEllipse(juce::Rectangle<float>(38, 38).withCentre(kp));
    g.setColour(juce::Colours::white);
    g.fillEllipse(juce::Rectangle<float>(28, 28).withCentre(kp));
    g.setColour(curCol);
    g.drawEllipse(juce::Rectangle<float>(28, 28).withCentre(kp), 3.2f);
    g.fillEllipse(juce::Rectangle<float>(9, 9).withCentre(kp));

    // center text like screenshot
    g.setColour(juce::Colours::white);
    g.setFont(juce::Font(52.0f, juce::Font::bold));
    g.drawText(juce::String((int)current) + "%", bounds.withSizeKeepingCentre(200, 80).translated(0, -10), juce::Justification::centred, false);

    g.setColour(juce::Colour::fromRGB(120, 116, 116));
    g.setFont(juce::Font(16.0f, juce::Font::plain));
    g.drawText("drag or arrow keys", bounds.withSizeKeepingCentre(200, 20).translated(0, 32), juce::Justification::centred, false);
}

KnobMuseAudioProcessorEditor::KnobMuseAudioProcessorEditor(KnobMuseAudioProcessor& p)
    : AudioProcessorEditor(p), proc(p), knob(p.apvts)
{
    addAndMakeVisible(knob);
    setSize(460, 460);
}
void KnobMuseAudioProcessorEditor::resized() { knob.setBounds(getLocalBounds()); }