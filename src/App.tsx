import assert from 'assert';
import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/slate/bootstrap.css';
import 'react-bootstrap-typeahead/css/Typeahead.css';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Button, Col, Container, Row, Spinner } from 'react-bootstrap';
import { Typeahead } from 'react-bootstrap-typeahead';
import { toast, ToastContainer } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipResultsParser,
  EventResultsParser,
  PaxService,
} from './services';
import { ChampionshipResults, ChampionshipType, EventResults } from './models';
import { EventResults as EventResultsComponent } from './components/EventResults';
import { FileUploadBox } from './components/FileUploadBox';
import { ChampionshipResults as ChampionshipResultsComponent } from './components/ChampionshipResults';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFiles: Record<ChampionshipType, File | undefined>;

  processing: boolean;

  eventResults?: EventResults;
  championshipResults?: ChampionshipResults;

  newLadies: string[];
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly paxService = new PaxService();
  private readonly eventResultsParser = new EventResultsParser();
  private readonly championshipResultsProcessor = new ChampionshipResultsParser(
    this.paxService,
  );
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {
      championshipResultsFiles: {
        Class: undefined,
        PAX: undefined,
        Novice: undefined,
        Ladies: undefined,
      },
      processing: false,
      newLadies: [],
    };
  }

  async componentDidMount() {
    try {
      await this.paxService.init();
    } catch (e) {
      console.error(e);
    }
  }

  render() {
    return (
      <div>
        <ToastContainer pauseOnHover />
        <Container fluid>
          {/* Page header */}
          <Row>
            <Col>
              <h1>SCCA Solo Points Calculator</h1>
            </Col>
          </Row>

          {/* File upload boxes */}
          <Row>
            <Col>
              <FileUploadBox
                label={'Full Event Results (by class)'}
                file={this.state.eventResultsFile}
                onFileSelect={async (f) => {
                  try {
                    await this.validateUploadedEventResultsFile(f);
                    const eventResults = await this.eventResultsParser.parse(
                      await f.text(),
                    );
                    this.setState({ eventResultsFile: f, eventResults });
                    return true;
                  } catch (e) {
                    console.error(e.message);
                    toast.error(
                      'File format does not match expected. Please export event results with raw times, grouped by class.',
                    );
                    return false;
                  }
                }}
                fileSelectedMessage={(f) => (
                  <p>
                    Ready to process <code>{f.name}</code> as new event results
                  </p>
                )}
              />

              <Typeahead
                placeholder={'Names of first-time ladies championship drivers'}
                disabled={!this.state.eventResults}
                options={Object.values(this.state.eventResults || {})
                  .map((classResults) => classResults.drivers)
                  .flat()
                  .map((driver) => driver.name)}
                multiple
                onChange={(newLadies) => {
                  this.setState({ newLadies });
                }}
              />
            </Col>

            <Col>
              {Object.keys(this.state.championshipResultsFiles).map(
                (championshipType, index) => (
                  <FileUploadBox
                    key={index}
                    label={`${championshipType} Championship Standings`}
                    file={
                      this.state.championshipResultsFiles[
                        championshipType as ChampionshipType
                      ]
                    }
                    onFileSelect={(f) => {
                      const newResults = {
                        ...this.state.championshipResultsFiles,
                      };
                      newResults[championshipType as ChampionshipType] = f;
                      this.setState({ championshipResultsFiles: newResults });
                      return true;
                    }}
                    fileSelectedMessage={(f) =>
                      this.state.eventResultsFile ? (
                        <p>
                          Ready to process <code>{f.name}</code> as{' '}
                          <strong>{championshipType}</strong> championship
                          standings
                        </p>
                      ) : (
                        <p>
                          <code>{f.name}</code> set as{' '}
                          <strong>{championshipType}</strong> championship
                          standings. Please add event results to begin
                          processing.
                        </p>
                      )
                    }
                  />
                ),
              )}
            </Col>
          </Row>

          {/* Process button */}
          <Row>
            <Col>
              <Button
                style={{ width: '150px' }}
                disabled={
                  Object.values(this.state.championshipResultsFiles).filter(
                    (v) => v,
                  ).length === 0
                }
                variant={'primary'}
                onClick={async () => {
                  this.setState({ processing: true });
                  const eventResults = await this.eventResultsParser.parse(
                    await this.state.eventResultsFile!.text(),
                  );
                  const championshipResults =
                    await this.championshipResultsProcessor.parse(
                      this.state.championshipResultsFiles,
                      eventResults,
                      this.state.newLadies,
                    );
                  this.setState({
                    eventResults,
                    championshipResults,
                    processing: false,
                  });
                }}
              >
                {this.state.processing ? (
                  <Spinner animation={'border'} />
                ) : (
                  <span>Process Championship</span>
                )}
              </Button>
            </Col>
          </Row>

          <EventResultsComponent
            paxService={this.paxService}
            results={this.state.eventResults}
          />

          <ChampionshipResultsComponent
            paxService={this.paxService}
            results={this.state.championshipResults}
          />
        </Container>
      </div>
    );
  }

  async validateUploadedEventResultsFile(f: File) {
    const content = await f.text();
    const rows: string[][] = parse(content, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
      skipEmptyLines: true,
    });
    const firstLine = rows[0];

    const errorMessage = `First line does not match expected value. Actual: \`"${firstLine.join(
      '"',
    )}"\``;

    assert(firstLine[0] === 'Results', errorMessage);
    assert(firstLine[2] === 'www.ProntoTimingSystem.com', errorMessage);
    assert(firstLine[3] === 'Pos', errorMessage);
    assert(firstLine[4] === 'Nbr', errorMessage);
    assert(firstLine[5] === "Driver's name, Town", errorMessage);
    assert(firstLine[6] === 'Car, Sponsor', errorMessage);
  }
}

export default App;
